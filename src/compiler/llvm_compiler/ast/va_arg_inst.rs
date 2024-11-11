use super::{Type, Value, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct VAArgInst { 
    pub lhs_type:Type,
    pub value: Value,
    pub rhs_type: Type,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl VAArgInst {
    pub fn new() -> VAArgInst {
        VAArgInst {
            lhs_type: Type::None,
            value: Value::None,
            rhs_type: Type::None,
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for VAArgInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VAArgInst {
        let mut item = VAArgInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
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
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
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
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}