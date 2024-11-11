use super::{Type, Ident};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum TypeKind {
    None,
    Opaque,
    Type {
        type_: Type
    }
}

#[derive(Debug)]
pub struct TypeDef {
    pub local_ident: Ident,
    pub kind: TypeKind
}

impl BuildFrom for TypeDef {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TypeDef {
        let mut type_def = TypeDef {
            local_ident: Ident::None,
            kind: TypeKind::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::LocalIdent => {
                    type_def.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::TypeKind => {
                    type_def.kind = TypeKind::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        type_def
    }
}

impl BuildFrom for TypeKind {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TypeKind {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Type => {
                        TypeKind::Type {
                            type_: Type::build_from(&inner_pair)
                        }
                    },
                    Rule::OpaqueType => TypeKind::Opaque,
                    _ => TypeKind::None
                }
            },
            None => TypeKind::None
        }
    }
}