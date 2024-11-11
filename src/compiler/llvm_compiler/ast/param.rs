use super::{ParamAttr, Ident, Type};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct Param { 
    pub type_: Type,
    pub param_attrs: Vec<ParamAttr>,
    pub local_ident: Ident,
    pub dot_dot_dot: bool
}


impl Param {
    pub fn new_dot_dot_dot() -> Param {
        Param {
            type_: Type::None,
            param_attrs: vec![],
            local_ident: Ident::None,
            dot_dot_dot: true
        }
    }
}


impl BuildFrom for Param {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Param {
        let mut param = Param {
            type_: Type::None,
            param_attrs: vec![],
            local_ident: Ident::None,
            dot_dot_dot: false
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    param.type_ = Type::build_from(&inner_pair);
                },
                Rule::LocalIdent => {
                    param.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::ParamAttrs => {
                    for p in inner_pair.into_inner() {
                        match p.as_rule() {
                            Rule::ParamAttr => {
                                param.param_attrs.push(ParamAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        param
    }
}
