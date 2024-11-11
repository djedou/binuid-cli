use super::{VectorOp, TypeValue, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct VectorInst { 
    pub op: VectorOp,
    pub lhs_type_value: TypeValue,
    pub rhs_type_value: TypeValue,
    pub type_value: TypeValue,
    pub metadata_attachments: Vec<MetadataAttachment>
}



impl VectorInst {
    pub fn new() -> VectorInst {
        VectorInst {
            op: VectorOp::None,
            lhs_type_value: TypeValue::new(),
            rhs_type_value: TypeValue::new(),
            type_value: TypeValue::new(),
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for VectorInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VectorInst {
        let mut item = VectorInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::VectorOp => {
                    item.op = VectorOp::build_from(&inner_pair);
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
                Rule::TypeValue => {
                    item.type_value = TypeValue::build_from(&inner_pair);
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