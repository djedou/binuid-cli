use super::{TypeValue, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct SelectInst { 
    pub lhs_type_value: TypeValue,
    pub rhs_type_value: TypeValue,
    pub type_value: TypeValue,
    pub metadata_attachments: Vec<MetadataAttachment>
}



impl SelectInst {
    pub fn new() -> SelectInst {
        SelectInst {
            lhs_type_value: TypeValue::new(),
            rhs_type_value: TypeValue::new(),
            type_value: TypeValue::new(),
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for SelectInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> SelectInst {
        let mut item = SelectInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::TypeValue => {
                    item.type_value = TypeValue::build_from(&inner_pair);
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
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}