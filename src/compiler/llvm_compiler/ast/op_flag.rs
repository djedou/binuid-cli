use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    ops::OpFlag,
    simples::OverflowFlag
};



impl BuildFrom for OpFlag {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> OpFlag {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::OverflowFlags => {
                        let mut list = vec![];
                        for p in inner_pair.into_inner() {
                            match p.as_rule() {
                                Rule::OverflowFlag => {
                                    list.push(OverflowFlag::build_from(&p));
                                },
                                _ => {}
                            }
                        }

                        OpFlag::Overflows {
                            flags: list
                        }
                    },
                    Rule::Exact => OpFlag::Exact,
                    _ => OpFlag::None
                }
            },
            None => OpFlag::None
        }
    }
}