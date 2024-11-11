use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct Section { 
    pub name: String
}


impl Section {
    pub fn new() -> Section {
        Section {
            name: String::with_capacity(0)
        }
    }
}

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