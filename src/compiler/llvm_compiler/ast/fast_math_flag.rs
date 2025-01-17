use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::FastMathFlag;



impl BuildFrom for FastMathFlag {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FastMathFlag {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Afn => FastMathFlag::Afn,
                    Rule::Arcp => FastMathFlag::Arcp,
                    Rule::Contract => FastMathFlag::Contract,
                    Rule::Fast => FastMathFlag::Fast,
                    Rule::Ninf => FastMathFlag::Ninf,
                    Rule::Nnan => FastMathFlag::Nnan,
                    Rule::Nsz => FastMathFlag::Nsz,
                    Rule::Reassoc => FastMathFlag::Reassoc,
                    _ => FastMathFlag::None
                }
            },
            None => FastMathFlag::None
        }
    }
}