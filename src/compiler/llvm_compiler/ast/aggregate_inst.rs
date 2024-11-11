use super::{AggregateOp, TypeValue, Index, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct AggregateInst {
    pub op: AggregateOp,
    pub lhs_type_value: TypeValue,
    pub rhs_type_value: TypeValue,
    pub indices: Vec<Index>,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl AggregateInst {
    pub fn new() -> AggregateInst {
        AggregateInst {
            op: AggregateOp::None,
            lhs_type_value: TypeValue::new(),
            rhs_type_value: TypeValue::new(),
            indices: vec![],
            metadata_attachments: vec![]
        }
    }
}

impl BuildFrom for AggregateInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AggregateInst {
        let mut item = AggregateInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::AggregateOp => {
                    item.op = AggregateOp::build_from(&inner_pair);
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.lhs_type_value = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs_type_value = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Indices => {
                    for p in inner_pair.into_inner() {
                        match p.as_rule() {
                            Rule::Index => {
                                item.indices.push(Index::build_from(&p));
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