use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct AddrSpace {
    pub addr: u32
}

impl AddrSpace {
    pub fn new() -> AddrSpace {
        AddrSpace {
            addr: 0
        }
    }
}

impl BuildFrom for AddrSpace {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AddrSpace {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::IntLit => {
                        AddrSpace {
                            addr: inner_pair.as_str().parse::<u32>().map_or(1, |d| d.clone())
                        }
                    },
                    _ => AddrSpace {
                        addr: 0
                    }
                }
            },
            None => AddrSpace {
                addr: 0
            }
        }
    }
}