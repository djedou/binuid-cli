use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    ops::AggregateOp,
    composes::{MetadataAttachment, Index},
    instructions::AggregateInst,
    types::TypeValue
};

impl BuildFrom for AggregateInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AggregateInst {
        let mut item = AggregateInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::AggregateOp => {
                    item.op = AggregateOp::build_from(&inner_pair);
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.lhs_type_value = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs_type_value = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Indices => {
                    for p in inner_pair.into_inner() {
                        match p.as_rule() {
                            Rule::Index => {
                                item.indices.push(Index::build_from(&p));
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