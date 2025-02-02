use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::Immutable;



impl BuildFrom for Immutable {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Immutable {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Const => Immutable::Const,
                    Rule::Glob => Immutable::Glob,
                    _ => Immutable::None
                }
            },
            None => Immutable::None
        }
    }
}