use crate::dwarf::export;
mod dwarf;

struct DwoParser;

impl dwarf::Guest for DwoParser {
    fn dwo_create(obj_binary: Vec<u8>) -> dwarf::DwoHandler {
        let dwo: Box<dwo_parser_impl::DwoParserImpl> =
            Box::new(dwo_parser_impl::DwoParserImpl::new(obj_binary));
        let handle: *mut dwo_parser_impl::DwoParserImpl = Box::into_raw(dwo);
        handle as u64
    }
    fn dwo_destroy(dwo: dwarf::DwoHandler) -> () {
        let _ = unsafe { Box::from_raw(dwo as *mut dwo_parser_impl::DwoParserImpl) };
    }

    fn dwo_get_line_map(dwo: dwarf::DwoHandler) -> String {
        let dwo: &mut dwo_parser_impl::DwoParserImpl =
            unsafe { &mut *(dwo as *mut dwo_parser_impl::DwoParserImpl) };
        dwo.get_line_map()
    }
}

export!(DwoParser with_types_in crate::dwarf);
