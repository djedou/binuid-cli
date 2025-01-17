use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::Auth;

impl BuildFrom for Auth {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Auth {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Readwrite => Auth::Readwrite,
                    Rule::Writeonly => Auth::Writeonly,
                    Rule::Readonly => Auth::Readonly,
                    Rule::Read => Auth::Read,
                    Rule::Write => Auth::Write,
                    _ => Auth::None
                }
            },
            None => Auth::None
        }
    }
}