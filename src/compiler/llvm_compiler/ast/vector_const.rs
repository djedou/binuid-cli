use super::TypeConst;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct VectorConst {
    pub value: Vec<TypeConst>
}

impl BuildFrom for VectorConst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VectorConst {
        let mut type_list = vec![];
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::TypeConst => {
                    type_list.push(TypeConst::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        VectorConst {
            value: type_list
        }
    }
}