use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::Alignment;




impl BuildFrom for Alignment {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Alignment {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::IntLit => {
                        Alignment {
                            int: inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone())
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