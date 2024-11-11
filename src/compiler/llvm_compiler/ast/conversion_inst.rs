use super::{OverflowFlag, ConversionOp, Type, Value, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum ConversionInstFlag {
    None,
    OverflowFlags {
        flags: Vec<OverflowFlag>
    },
    Nneg
}


#[derive(Debug)]
pub struct ConversionInst {
    pub op: ConversionOp,
    pub flag: ConversionInstFlag,
    pub lhs_type: Type,
    pub value: Value,
    pub rhs_type: Type,
    pub metadata_attachments: Vec<MetadataAttachment>
}



impl ConversionInst {
    pub fn new() -> ConversionInst {
        ConversionInst {
            op: ConversionOp::None,
            flag: ConversionInstFlag::None,
            lhs_type: Type::None,
            value: Value::None,
            rhs_type: Type::None,
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for ConversionInstFlag {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ConversionInstFlag {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::OverflowFlags => {
                        let mut flags = vec![];
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::OverflowFlag => {
                                    flags.push(OverflowFlag::build_from(&p));
                                },
                                _ => {}
                            }
                        }

                        ConversionInstFlag::OverflowFlags {
                            flags: flags
                        }
                    },
                    Rule::Nneg => ConversionInstFlag::Nneg,
                    _ => ConversionInstFlag::None
                }
            },
            None => ConversionInstFlag::None
        }
    }
}

impl BuildFrom for ConversionInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ConversionInst {
        let mut item = ConversionInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ConversionOp => {
                    item.op = ConversionOp::build_from(&inner_pair);
                },
                Rule::ConversionInstFlag => {
                    item.flag = ConversionInstFlag::build_from(&inner_pair);
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