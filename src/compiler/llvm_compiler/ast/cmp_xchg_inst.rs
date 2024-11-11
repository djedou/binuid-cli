use super::{TypeValue, AtomicOrdering, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct CmpXchgInst { 
    pub weak: bool,
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub type_value: TypeValue,
    pub lhs_order: AtomicOrdering,
    pub rhs_order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl CmpXchgInst {
    pub fn new() -> CmpXchgInst {
        CmpXchgInst {
            weak: false,
            volatile: false,
            lhs: TypeValue::new(),
            rhs: TypeValue::new(),
            type_value: TypeValue::new(),
            lhs_order: AtomicOrdering::None,
            rhs_order: AtomicOrdering::None,
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for CmpXchgInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CmpXchgInst {
        let mut item = CmpXchgInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Weak => {
                    item.weak = true;
                },
                Rule::Volatile => {
                    item.volatile = true;
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
                Rule::TypeValue => {
                    item.type_value = TypeValue::build_from(&inner_pair);
                },
                Rule::LhsAtomicOrdering => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::AtomicOrdering => {
                                item.lhs_order = AtomicOrdering::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsAtomicOrdering => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::AtomicOrdering => {
                                item.rhs_order = AtomicOrdering::build_from(&p);
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