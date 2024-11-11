use super::{Type, Inc, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct PhiInst {
    pub type_: Type,
    pub inc_list: Vec<Inc>,
    pub metadata_attachments: Vec<MetadataAttachment>
}



impl PhiInst {
    pub fn new() -> PhiInst {
        PhiInst {
            type_: Type::None,
            inc_list: vec![],
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for PhiInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> PhiInst {
        let mut item = PhiInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::IncList => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Inc => {
                                item.inc_list.push(Inc::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}