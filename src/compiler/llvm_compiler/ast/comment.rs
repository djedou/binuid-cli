use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct Comment {
    pub value: String
}


impl BuildFrom for Comment {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Comment {
        match pair.clone().into_inner().next() {
            Some(p) => {
                Comment {
                    value: p.as_str().trim().to_string()
                }
            },
            None => {
                Comment {
                    value: String::with_capacity(0)
                }
            }
        }
    }
}