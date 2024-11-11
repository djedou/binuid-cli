use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct MetadataId {
    value: u32
}


impl MetadataId {
    pub fn new() -> MetadataId {
        MetadataId {
            value: 0
        }
    }
}


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