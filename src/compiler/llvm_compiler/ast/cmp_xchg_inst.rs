use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::AtomicOrdering;
use binuid_shared_wasm::ast_bits::instructions::CmpXchgInst;
use binuid_shared_wasm::ast_bits::types::TypeValue;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;



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