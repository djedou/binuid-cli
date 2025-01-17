use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::Alignment,
    composes::{Section, MetadataAttachment, Comdat, GlobalAttr}
};



impl BuildFrom for GlobalAttr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> GlobalAttr {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Section => GlobalAttr::Section {
                        section: Section::build_from(&inner_pair)
                    },
                    /*
                    Rule::Comdat => GlobalAttr::Comdat {
                        comdat: Comdat
                    },
                    */
                    Rule::Alignment => GlobalAttr::Align {
                        align: Alignment::build_from(&inner_pair)
                    },
                    Rule::MetadataAttachment => GlobalAttr::MetadataAttachment {
                        attachment: MetadataAttachment::build_from(&inner_pair)
                    },
                    _ => GlobalAttr::None
                }
            },
            None => GlobalAttr::None
        }
    }
}