use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::Auth,
    composes::{MemoryArg, Ident, MemoryArgItem}
};

impl BuildFrom for MemoryArgItem {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MemoryArgItem {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::MemoryArgs => {
                        let mut args = vec![];
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::MemoryArg => {
                                    args.push(MemoryArg::build_from(&p));
                                },
                                _ => {}
                            }
                        }

                        MemoryArgItem::Args {
                            args: args
                        }
                    },
                    _ => MemoryArgItem::None
                }
            },
            None => MemoryArgItem::None
        }
    }
}

impl BuildFrom for MemoryArg {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MemoryArg {
        let mut item = MemoryArg::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::LabelIdent => {
                    item.label_ident = Ident::build_from(&inner_pair);
                },
                Rule::Auth => {
                    item.auth = Auth::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}