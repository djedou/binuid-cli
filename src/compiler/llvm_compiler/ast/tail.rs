use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::Tail;

impl BuildFrom for Tail {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Tail {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Musttail => Tail::Must,
                    Rule::Notail => Tail::No,
                    Rule::TailKey => Tail::Key,
                    _ => Tail::None
                }
            },
            None => Tail::None
        }
    }
}