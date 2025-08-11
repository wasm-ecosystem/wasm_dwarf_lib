#![warn(dead_code)]

use crate::dwarf::export;
use std::{cell::RefCell, error, rc::Rc};
mod dwarf;
use object::{Object, ObjectSection};

struct DwarfScanner;

struct DwoParserImpl {
    binary: Vec<u8>,
    dwarf: Option<gimli::DwarfSections<gimli::EndianRcSlice<gimli::RunTimeEndian>>>,
}

impl DwoParserImpl {
    fn init_dwarf(self: &mut Self) {
        if self.dwarf.is_none() {
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

    fn get_line_map(self: &mut Self) -> String {
        self.init_dwarf();
        String::new()
    }
}

struct DwoParser {
    implement: RefCell<DwoParserImpl>,
}

impl dwarf::exports::wasm_ecosystem::dwarf::dwarf_parser::GuestDwo for DwoParser {
    fn new(obj_binary: Vec<u8>) -> Self {
        Self {
            implement: RefCell::new(DwoParserImpl {
                binary: obj_binary,
                dwarf: None,
            }),
        }
    }

    fn get_line_map(&self) -> String {
        self.implement.borrow_mut().get_line_map()
    }

    fn list_cu(&self) -> String {
        todo!()
    }

    fn list_subprograms(&self, cu: String) -> String {
        todo!()
    }
}

struct ExportedDwoParser;

impl dwarf::exports::wasm_ecosystem::dwarf::dwarf_parser::Guest for ExportedDwoParser {
    type Dwo = DwoParser;
}

export!(ExportedDwoParser with_types_in crate::dwarf);
