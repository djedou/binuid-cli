use super::{FuncAttr, Ident};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct AttrGroupDef {
    pub attr_group_id: Ident,
    pub func_attrs: Vec<FuncAttr>
}


impl AttrGroupDef {
    pub fn new() -> AttrGroupDef {
        AttrGroupDef {
            attr_group_id: Ident::None,
            func_attrs: vec![]
        }
    }
}


impl BuildFrom for AttrGroupDef {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AttrGroupDef {
        let mut item = AttrGroupDef::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::AttrGroupId => {
                    item.attr_group_id = Ident::build_from(&inner_pair);
                },
                Rule::FuncAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::FuncAttr => {
                                item.func_attrs.push(FuncAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        item
    }
}