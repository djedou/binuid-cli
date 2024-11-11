use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct Alignment {
    pub int: u32
}

impl Alignment {
    pub fn new() -> Alignment {
        Alignment {
            int: 0
        }
    }
}

impl BuildFrom for Alignment {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Alignment {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::IntLit => {
                        Alignment {
                            int: inner_pair.as_str().parse::<u32>().map_or(1, |d| d.clone())
                        }
                    },
                    _ => Alignment {
                        int: 1
                    }
                }
            },
            None => Alignment {
                int: 1
            }
        }
    }
}