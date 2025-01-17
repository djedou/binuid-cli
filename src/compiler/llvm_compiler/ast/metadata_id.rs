use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::MetadataId;

impl BuildFrom for MetadataId {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MetadataId {
        let mut metadata = MetadataId::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Id => {
                    metadata.value = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                _ => {}
            }
        }

        metadata
    }
}