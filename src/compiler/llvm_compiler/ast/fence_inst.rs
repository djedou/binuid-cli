use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::AtomicOrdering;
use binuid_shared_wasm::ast_bits::instructions::FenceInst;
use binuid_shared_wasm::ast_bits::composes::MetadataAttachment;



impl BuildFrom for FenceInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FenceInst {
        let mut item = FenceInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
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