use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct MetadataName {
    value: String
}

impl MetadataName {
    pub fn new() -> MetadataName {
        MetadataName {
            value: String::with_capacity(0)
        }
    }
}


impl BuildFrom for MetadataName {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MetadataName {
        let mut metadata = MetadataName::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::EscapeName => {
                    metadata.value = String::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        metadata
    }
}