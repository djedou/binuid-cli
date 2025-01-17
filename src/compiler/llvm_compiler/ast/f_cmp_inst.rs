use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::instructions::FCmpInst;
use binuid_shared_wasm::ast_bits::simples::{FPred, FastMathFlag};
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;





impl BuildFrom for FCmpInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FCmpInst {
        let mut item = FCmpInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::FastMathFlags => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::FastMathFlag => {
                                item.flags.push(FastMathFlag::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::FPred => {
                    item.f_pred = FPred::build_from(&inner_pair);
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