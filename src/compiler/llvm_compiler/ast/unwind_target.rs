use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{Ident, UnwindTarget};


impl BuildFrom for UnwindTarget {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> UnwindTarget {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ToCaller => UnwindTarget::ToCaller,
                    Rule::LocalLabel => UnwindTarget::LocalLabel {
                        label: Ident::build_from(&inner_pair)
                    },
                    _ => UnwindTarget::None
                }
            },
            None => UnwindTarget::None
        }
    }
}