use super::TypeConst;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum StructConst {
    None,
    GrOrLessParath {
        type_list: Vec<TypeConst>
    },
    GrOrLessParathEmpty,
    OnlyParath {
        type_list: Vec<TypeConst>
    },
    OnlyParathEmpty
}


impl BuildFrom for StructConst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> StructConst {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::GrOrLessParath => {
                        let mut type_list = vec![];
                        for p in inner_pair.into_inner() {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    type_list.push(TypeConst::build_from(&p));
                                },
                                _ => {}
                            }
                        }
                        
                        StructConst::GrOrLessParath {
                            type_list: type_list
                        }
                    },
                    Rule::GrOrLessParathEmpty => StructConst::GrOrLessParathEmpty,
                    Rule::OnlyParath => {
                        let mut type_list = vec![];
                        for p in inner_pair.into_inner() {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    type_list.push(TypeConst::build_from(&p));
                                },
                                _ => {}
                            }
                        }
                        
                        StructConst::OnlyParath {
                            type_list: type_list
                        }
                    },
                    Rule::OnlyParathEmpty => StructConst::OnlyParathEmpty,
                    _ => StructConst::None
                }
            },
            None => StructConst::None
        }
    }
}