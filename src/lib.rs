use crate::dwarf::export;
use serde_json::json;
use std::borrow;
use std::error;
mod dwarf;
use object::{Object, ObjectSection};

struct DwarfScanner;

impl dwarf::Guest for DwarfScanner {
    fn scan(binary: Vec<u8>) -> String {
        let mut results = Vec::new();

        let object = object::File::parse(&*binary).expect("Failed to parse object file");
        let load_section =
            |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, Box<dyn error::Error>> {
                Ok(match object.section_by_name(id.name()) {
                    Some(section) => section.uncompressed_data()?,
                    None => borrow::Cow::Borrowed(&[]),
                })
            };
        let dwarf_sections =
            gimli::DwarfSections::load(&load_section).expect("load dwarf sections failed");
        let borrow_section = |section| {
            gimli::EndianSlice::new(borrow::Cow::as_ref(section), gimli::RunTimeEndian::Little)
        };
        let dwarf = dwarf_sections.borrow(borrow_section);
        let mut iter = dwarf.units();
        while let Some(header) = iter.next().unwrap() {
            let unit = dwarf.unit(header).unwrap();
            let unit = unit.unit_ref(&dwarf);
            if let Some(program) = unit.line_program.clone() {
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
            }
        }
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }
}

export!(DwarfScanner with_types_in crate::dwarf);

#[cfg(test)]
mod tests {
    use crate::dwarf::Guest;

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
            )
        "#;
        let wasm = wat::Parser::new()
            .generate_dwarf(wat::GenerateDwarf::Lines)
            .parse_str(None, wat)
            .expect("generating wasm failed");
        let result = super::DwarfScanner::scan(wasm);
        assert_eq!(
            result,
            r#"[{"address":2,"line":3},{"address":4,"line":4},{"address":5,"line":4},{"address":8,"line":7},{"address":10,"line":8},{"address":12,"line":9},{"address":13,"line":10},{"address":15,"line":11},{"address":16,"line":11}]"#
        );
    }
}
