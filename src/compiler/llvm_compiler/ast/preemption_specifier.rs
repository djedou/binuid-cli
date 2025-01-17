use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::PreemptionSpecifier;


impl BuildFrom for PreemptionSpecifier {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> PreemptionSpecifier {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::DsoLocal => PreemptionSpecifier::DsoLocal,
                    Rule::DsoPreemptable => PreemptionSpecifier::DsoPreemptable,
                    _ => PreemptionSpecifier::None
                }
            },
            None => PreemptionSpecifier::None
        }
    }
}