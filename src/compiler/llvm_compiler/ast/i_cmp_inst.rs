use super::{IPred, Type, Value, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct ICmpInst { 
    pub i_pred: IPred,
    pub type_: Type,
    pub lhs_value: Value,
    pub rhs_value: Value,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl ICmpInst {
    pub fn new() -> ICmpInst {
        ICmpInst {
            i_pred: IPred::None,
            type_: Type::None,
            lhs_value: Value::None,
            rhs_value: Value::None,
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for ICmpInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ICmpInst {
        let mut item = ICmpInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::IPred => {
                    item.i_pred = IPred::build_from(&inner_pair);
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::LhsValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Value => {
                                item.lhs_value = Value::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Value => {
                                item.rhs_value = Value::build_from(&p);
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