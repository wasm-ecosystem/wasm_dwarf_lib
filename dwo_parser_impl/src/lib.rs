use object::{Object, ObjectSection};
use serde_json::json;
use std::{error, rc::Rc};

pub struct DwoParserImpl {
    pub binary: Vec<u8>,
    pub dwarf: Option<gimli::DwarfSections<gimli::EndianRcSlice<gimli::RunTimeEndian>>>,
}

impl DwoParserImpl {
    pub fn new(obj_binary: Vec<u8>) -> Self {
        Self {
            binary: obj_binary,
            dwarf: None,
        }
    }
    pub fn init_dwarf(&mut self) {
        if self.dwarf.is_some() {
            return;
        }
        let object: object::File =
            object::File::parse(&*self.binary).expect("Failed to parse object file");
        let dwarf_sections: gimli::DwarfSections<gimli::EndianRcSlice<gimli::RunTimeEndian>> =
            gimli::DwarfSections::load(&|id: gimli::SectionId| -> Result<
                gimli::EndianRcSlice<gimli::RunTimeEndian>,
                Box<dyn error::Error>,
            > {
                let data: Rc<[u8]> = match object.section_by_name(id.name()) {
                    Some(section) => Rc::<[u8]>::from(section.uncompressed_data()?.into_owned()),
                    None => Rc::new([]),
                };
                Ok(gimli::EndianRcSlice::<gimli::RunTimeEndian>::new(
                    data,
                    gimli::RunTimeEndian::Little,
                ))
            })
            .expect("load dwarf sections failed");
        self.dwarf = Some(dwarf_sections);
    }

    pub fn get_line_map(&mut self) -> String {
        self.init_dwarf();
        let mut results = Vec::<serde_json::Value>::new();
        let debug_line = &self.dwarf.as_ref().unwrap().debug_line;
        let offset = gimli::DebugLineOffset(0);
        let address_size = 8;
        let program = debug_line
            .program(offset, address_size, None, None)
            .expect("should have found a header at that offset, and parsed it OK");
        let mut rows = program.rows();
        while let Some((_, row)) = rows.next_row().unwrap() {
            if row.end_sequence() {
                continue;
            }
            let line = match row.line() {
                Some(line) => line.get() - 1, // convention: 0-based line numbers
                None => continue,
            };
            let address = row.address() as u32;
            results.push(json!({"line": line, "address": address}));
        }
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_with_real_dwo_file() {
        let wat = r#"
          (module
              (func
                  call $bar
                  drop
              )
              (func $bar (result i32)
                  i32.const 1
                  i32.const 2
                  i32.add
                  i32.const 3
                  i32.add
              )
          )"#;
        let wasm = wat::Parser::new()
            .generate_dwarf(wat::GenerateDwarf::Lines)
            .parse_str(None, wat)
            .expect("generating wasm failed");
        let mut parser = DwoParserImpl::new(wasm);
        let result = parser.get_line_map();
        assert_eq!(
            result,
            r#"[{"address":2,"line":3},{"address":4,"line":4},{"address":5,"line":4},{"address":8,"line":7},{"address":10,"line":8},{"address":12,"line":9},{"address":13,"line":10},{"address":15,"line":11},{"address":16,"line":11}]"#
        );
    }
}
