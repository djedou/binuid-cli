use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::ops::{OpFlag, BinaryOp};
use binuid_shared_wasm::ast_bits::instructions::BinaryInst;
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;




impl BuildFrom for BinaryInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BinaryInst {
        let mut item = BinaryInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::BinaryOp => {
                    item.op = BinaryOp::build_from(&inner_pair);
                },
                Rule::OpFlag => {
                    item.flag = OpFlag::build_from(&inner_pair);
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