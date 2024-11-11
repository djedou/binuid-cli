use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Ident {
    None,
    Name {
        value: String
    },
    Id {
        value: u32
    }
}

impl BuildFrom for Ident {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Ident {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::QuotedName | Rule::Name => {
                        Ident::Name { 
                            value: String::build_from(&inner_pair)
                        } 
                    },
                    Rule::Id => {
                        inner_pair.as_str().parse::<u32>().map_or(Ident::None, |d| 
                            Ident::Id {
                                value: d.clone()
                            }
                        )
                    },
                    _ => Ident::None
                }
            },
            None => Ident::None
        }
    }
}