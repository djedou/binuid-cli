use super::Type;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct Sret {
    pub type_: Type
}

impl BuildFrom for Sret {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Sret {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Type => Sret {
                            type_: Type::build_from(&inner_pair)
                    },
                    _ => Sret {
                        type_: Type::None
                    }
                }
            },
            None => Sret {
                type_: Type::None
            }
        }
    }
}