use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::TargetKind,
    composes::TargetDefinition
};


impl BuildFrom for TargetDefinition {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TargetDefinition {
        let mut target = TargetDefinition {
            kind: TargetKind::None,
            data: String::with_capacity(0)
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Datalayout => {
                    target.kind = TargetKind::Datalayout;
                },
                Rule::Triple => {
                    target.kind = TargetKind::Triple;
                },
                Rule::StringLit => {
                    target.data = String::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        target
    }
}