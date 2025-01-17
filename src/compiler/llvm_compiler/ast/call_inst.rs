use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::{CallingConv, Tail, FastMathFlag};
use binuid_shared_wasm::ast_bits::instructions::CallInst;
use binuid_shared_wasm::ast_bits::composes::{Arg, ReturnAttr, FuncAttr, OperandBundle, MetadataAttachment};
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::values::Value;






impl BuildFrom for CallInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CallInst {
        let mut item = CallInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Tail => {
                    item.tail = Tail::build_from(&inner_pair);
                },
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
                Rule::CallingConv => {
                    item.calling_conv = CallingConv::build_from(&inner_pair);
                },
                Rule::ReturnAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::ReturnAttr => {
                                item.return_attrs.push(ReturnAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::Args => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Arg => {
                                item.args.push(Arg::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::FuncAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::FuncAttr => {
                                item.func_attrs.push(FuncAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::OperandBundles => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::OperandBundle => {
                                item.operand_bundles.push(OperandBundle::build_from(&p));
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