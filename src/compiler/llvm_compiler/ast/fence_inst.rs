use super::{MetadataAttachment, AtomicOrdering};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct FenceInst { 
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl FenceInst {
    pub fn new() -> FenceInst {
        FenceInst {
            order: AtomicOrdering::None,
            metadata_attachments: Vec::with_capacity(0)
        }
    }
}



impl BuildFrom for FenceInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FenceInst {
        let mut item = FenceInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::AtomicOrdering => {
                    item.order = AtomicOrdering::build_from(&inner_pair);
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