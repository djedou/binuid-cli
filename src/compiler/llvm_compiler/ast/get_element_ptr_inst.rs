use super::{MemoryOp, Type, Value, TypeValue, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct GetElementPtrInst {
    pub op: MemoryOp,
    pub in_bounds: bool,
    pub lhs_type: Type,
    pub rhs_type: Type,
    pub value: Value,
    pub type_values: Vec<TypeValue>,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl GetElementPtrInst {
    pub fn new() -> GetElementPtrInst {
        GetElementPtrInst {
            op: MemoryOp::None,
            in_bounds: false,
            lhs_type: Type::None,
            rhs_type: Type::None,
            value: Value::None,
            type_values: vec![],
            metadata_attachments: vec![]
        }
    }
}

impl BuildFrom for GetElementPtrInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> GetElementPtrInst {
        let mut item = GetElementPtrInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MemoryOp => {
                    item.op = MemoryOp::build_from(&inner_pair);
                },
                Rule::InBounds => {
                    item.in_bounds = false;
                },
                Rule::LhsType => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Type => {
                                item.lhs_type = Type::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsType => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Type => {
                                item.rhs_type = Type::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::CommaSepTypeValueList => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.type_values.push(TypeValue::build_from(&p));
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