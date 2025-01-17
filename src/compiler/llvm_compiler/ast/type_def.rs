use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::types::TypeKind;
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::composes::Ident;
use binuid_shared_wasm::ast_bits::types::TypeDef;



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