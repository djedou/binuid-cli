use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::instructions::GetElementPtrInst;
use binuid_shared_wasm::ast_bits::ops::MemoryOp;
use binuid_shared_wasm::ast_bits::types::{TypeValue, Type};
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;





impl BuildFrom for GetElementPtrInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> GetElementPtrInst {
        let mut item = GetElementPtrInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MemoryOp => {
                    item.op = MemoryOp::build_from(&inner_pair);
                },
                Rule::InBounds => {
                    item.in_bounds = false;
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
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::CommaSepTypeValueList => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.type_values.push(TypeValue::build_from(&p));
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