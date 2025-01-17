use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::Alignment,
    composes::{ReturnAttr, Range, Dereferenceable}
};


impl BuildFrom for ReturnAttr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ReturnAttr {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Alignment => ReturnAttr::Align {
                        align: Alignment::build_from(&inner_pair)
                    },
                    Rule::Dereferenceable => ReturnAttr::Dereferenceable {
                        deref: Dereferenceable::build_from(&inner_pair)
                    },
                    Rule::StringLit => ReturnAttr::String {
                        data: String::build_from(&inner_pair)
                    },
                    Rule::Inreg => ReturnAttr::Inreg,
                    Rule::Noalias => ReturnAttr::Noalias,
                    Rule::Nonnull => ReturnAttr::Nonnull,
                    Rule::Signext => ReturnAttr::Signext,
                    Rule::Zeroext => ReturnAttr::Zeroext,
                    Rule::Noundef => ReturnAttr::Noundef,
                    Rule::Range => ReturnAttr::Range {
                        range: Range::build_from(&inner_pair)
                    },
                    _ => ReturnAttr::None
                }
            },
            None => ReturnAttr::None
        }
    }
}