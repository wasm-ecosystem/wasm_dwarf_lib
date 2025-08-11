use crate::dwarf::export;
use std::cell::RefCell;
mod dwarf;
use dwo_parser_impl::DwoParserImpl;

#[allow(dead_code)]
struct DwarfScanner;

struct DwoParser {
    implement: RefCell<dwo_parser_impl::DwoParserImpl>,
}

impl dwarf::exports::wasm_ecosystem::dwarf::dwarf_parser::GuestDwo for DwoParser {
    fn new(obj_binary: Vec<u8>) -> Self {
        Self {
            implement: RefCell::new(dwo_parser_impl::DwoParserImpl::new(obj_binary)),
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
