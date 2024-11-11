use super::{MetadataAttachment, Comdat, Alignment, Section};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum GlobalAttr {
    None,
    Section {
        section: Section
    },
	Comdat {
        comdat: Comdat
    },
	Align {
        align: Alignment
    },
	MetadataAttachment {
        attachment: MetadataAttachment
    }
}

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