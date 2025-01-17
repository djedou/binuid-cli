use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::AtomicOrdering;
use binuid_shared_wasm::ast_bits::ops::BinOp;
use binuid_shared_wasm::ast_bits::instructions::AtomicRMWInst;
use binuid_shared_wasm::ast_bits::types::TypeValue;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;


impl BuildFrom for AtomicRMWInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AtomicRMWInst {
        let mut item = AtomicRMWInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    item.volatile = true;
                },
                Rule::BinOp => {
                    item.bin_op = BinOp::build_from(&inner_pair);
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
                Rule::AtomicOrdering => {
                    item.order = AtomicOrdering::build_from(&inner_pair);
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