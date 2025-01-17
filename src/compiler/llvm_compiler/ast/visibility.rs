use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::Visibility;


impl BuildFrom for Visibility {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Visibility {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Default => Visibility::Default,
                    Rule::Hidden => Visibility::Hidden,
                    Rule::Protected => Visibility::Protected,
                    _ => Visibility::None
                }
            },
            None => Visibility::None
        }
    }
}