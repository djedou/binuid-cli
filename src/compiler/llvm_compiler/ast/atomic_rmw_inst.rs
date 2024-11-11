use super::{BinOp, TypeValue, AtomicOrdering, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct AtomicRMWInst { 
    pub volatile: bool,
    pub bin_op: BinOp,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl AtomicRMWInst {
    pub fn new() -> AtomicRMWInst {
        AtomicRMWInst {
            volatile: false,
            bin_op: BinOp::None,
            lhs: TypeValue::new(),
            rhs: TypeValue::new(),
            order: AtomicOrdering::None,
            metadata_attachments: vec![]
        }
    }
}

impl BuildFrom for AtomicRMWInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AtomicRMWInst {
        let mut item = AtomicRMWInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    item.volatile = true;
                },
                Rule::BinOp => {
                    item.bin_op = BinOp::build_from(&inner_pair);
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.lhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
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