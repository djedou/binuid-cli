use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::AtomicOrdering;



impl BuildFrom for AtomicOrdering {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AtomicOrdering {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::AcqRel => AtomicOrdering::AcqRel,
                    Rule::Acquire => AtomicOrdering::Acquire,
                    Rule::Monotonic => AtomicOrdering::Monotonic,
                    Rule::Release => AtomicOrdering::Release,
                    Rule::SeqCst => AtomicOrdering::SeqCst,
                    Rule::Unordered => AtomicOrdering::Unordered,
                    _ => AtomicOrdering::None
                }
            },
            None => AtomicOrdering::None
        }
    }
}