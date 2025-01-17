use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::Range;
use binuid_shared_wasm::ast_bits::types::Type;

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