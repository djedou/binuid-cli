use super::TypeConst;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct ArrayConst {
    pub value: Vec<TypeConst>
}


impl ArrayConst {
    pub fn new() -> ArrayConst {
        ArrayConst {
            value: vec![]
        }
    }
}


impl BuildFrom for ArrayConst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ArrayConst {
        let mut type_list = vec![];
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::TypeConst => {
                    type_list.push(TypeConst::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        ArrayConst {
            value: type_list
        }
    }
}