use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::AddrSpace;



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