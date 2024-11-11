use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct MDString {
    value: String
}


impl MDString {
    pub fn new() -> MDString {
        MDString {
            value: String::with_capacity(0)
        }
    }
}


impl BuildFrom for MDString {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MDString {
        let mut metadata = MDString::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Id => {
                    metadata.value = String::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        metadata
    }
}