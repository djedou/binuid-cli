use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::Section;


impl BuildFrom for Section {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Section {
        let mut section = Section::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::StringLit => {
                    section.name = String::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        section
    }
}