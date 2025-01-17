use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::DereferenceableItem,
    composes::Dereferenceable
};



impl BuildFrom for Dereferenceable {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Dereferenceable {
        let mut der = Dereferenceable {
            item: DereferenceableItem::None,
            value: 0
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::DereferenceableNullable => {
                    der.item = DereferenceableItem::Nullable
                },
                Rule::DereferenceableNonNullable => {
                    der.item = DereferenceableItem::NonNullable
                },
                Rule::IntLit => {
                    der.value = inner_pair.as_str().parse::<u32>().map_or(1, |d| d.clone())
                },
                _ => {}
            }
        }

        der
    }
}