use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    ops::{BitwiseOp, OpFlag},
    values::Value,
    types::Type,
    composes::MetadataAttachment,
    instructions::BitwiseInst
};




impl BuildFrom for BitwiseInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BitwiseInst {
        let mut item = BitwiseInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::BitwiseOp => {
                    item.op = BitwiseOp::build_from(&inner_pair);
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