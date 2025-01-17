use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::OverflowFlag;
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::instructions::{ConversionInst, ConversionInstFlag};
use binuid_shared_wasm::ast_bits::ops::ConversionOp;
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;



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