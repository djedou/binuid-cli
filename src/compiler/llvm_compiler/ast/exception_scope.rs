use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{Ident, ExceptionScope};



impl BuildFrom for ExceptionScope {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ExceptionScope {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::LocalIdent => ExceptionScope::Local {
                        ident: Ident::build_from(&inner_pair)
                    },
                    _ => ExceptionScope::None
                }
            },
            None => ExceptionScope::None
        }
    }
}