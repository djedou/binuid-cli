use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::IPred;



impl BuildFrom for IPred {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> IPred {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Eq => IPred::Eq,
                    Rule::Ne => IPred::Ne,
                    Rule::Sge => IPred::Sge,
                    Rule::Sgt => IPred::Sgt,
                    Rule::Sle => IPred::Sle,
                    Rule::Slt => IPred::Slt,
                    Rule::Uge => IPred::Uge,
                    Rule::Ugt => IPred::Ugt,
                    Rule::Ule => IPred::Ule,
                    Rule::Ult => IPred::Ult,
                    _ => IPred::None
                }
            },
            None => IPred::None
        }
    }
}