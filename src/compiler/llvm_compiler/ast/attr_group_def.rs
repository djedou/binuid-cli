use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{Ident, AttrGroupDef, FuncAttr};




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