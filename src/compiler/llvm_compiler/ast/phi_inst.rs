use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::instructions::PhiInst;
use binuid_shared_wasm::ast_bits::composes::{MetadataAttachment, Inc};
use binuid_shared_wasm::ast_bits::types::Type;


impl BuildFrom for PhiInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> PhiInst {
        let mut item = PhiInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::IncList => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Inc => {
                                item.inc_list.push(Inc::build_from(&p));
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