use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct TargetDefinition {
    pub kind: TargetKind,
    pub data: String
}

#[derive(Debug)]
pub enum TargetKind {
    None,
    Datalayout,
    Triple
}

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