use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum Comdat {
    None,
    Name {
        name: String
    }
}


impl BuildFrom for Comdat {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Comdat {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ComdatName => Comdat::Name {
                        name: String::build_from(&inner_pair)
                    },
                    _ => Comdat::None
                }
            },
            None => Comdat::None
        }
    }
}