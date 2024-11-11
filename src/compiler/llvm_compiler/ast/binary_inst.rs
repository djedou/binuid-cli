use super::{BinaryOp, OpFlag, Type, Value, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct BinaryInst { 
    pub op: BinaryOp,
    pub flag: OpFlag,
    pub type_: Type,
    pub lhs_value: Value,
    pub rhs_value: Value,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl BinaryInst {
    pub fn new() -> BinaryInst {
        BinaryInst {
            op: BinaryOp::None,
            flag: OpFlag::None,
            type_: Type::None,
            lhs_value: Value::None,
            rhs_value: Value::None,
            metadata_attachments: vec![]
        }
    }
}

impl BuildFrom for BinaryInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BinaryInst {
        let mut item = BinaryInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::BinaryOp => {
                    item.op = BinaryOp::build_from(&inner_pair);
                },
                Rule::OpFlag => {
                    item.flag = OpFlag::build_from(&inner_pair);
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