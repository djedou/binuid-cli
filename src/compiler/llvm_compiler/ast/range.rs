use super::Type;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct Range {
    pub type_: Type,
    pub int: u32,
    pub int_const: i64
}


impl BuildFrom for Range {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Range {
        let mut range = Range {
            type_: Type::None,
            int: 0,
            int_const: 0
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    range.type_ = Type::build_from(&inner_pair);
                },
                Rule::IntLit => {
                    range.int = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                Rule::IntConst => {
                    range.int_const = inner_pair.as_str().parse::<i64>().map_or(0, |d| d.clone());
                },
                _ => {}
            }
        }

        range
    }
}