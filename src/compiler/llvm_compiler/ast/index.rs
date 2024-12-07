use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct Index {
    pub value: u32
}

impl BuildFrom for Index {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Index {
        let mut index = Index {
            value: 0
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::IntLit => {
                    index.value = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                _ => {}
            }
        }

        index
    }
}