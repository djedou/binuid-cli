use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::TLSModel;


impl BuildFrom for TLSModel {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TLSModel {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::InitialExec => TLSModel::InitialExec,
                    Rule::LocalDynamic => TLSModel::LocalDynamic,
                    Rule::LocalExec => TLSModel::LocalExec,
                    _ => TLSModel::None
                }
            },
            None => TLSModel::None
        }
    }
}