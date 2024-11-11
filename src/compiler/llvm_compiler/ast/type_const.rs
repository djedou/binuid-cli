use super::{Type, Constant};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct TypeConst { 
    type_: Type,
    constant: Constant
}

impl TypeConst {
    pub fn new() -> TypeConst {
        TypeConst { 
            type_: Type::None,
            constant: Constant::None
        }
    }
}


impl BuildFrom for TypeConst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TypeConst {
        let mut type_const = TypeConst {
            type_: Type::None,
            constant: Constant::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    type_const.type_ = Type::build_from(&inner_pair);
                },
                Rule::Constant => {
                    type_const.constant = Constant::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        type_const
    }
}