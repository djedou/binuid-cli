use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    composes::MetadataAttachment,
    types::TypeValue
};
use binuid_shared_wasm::ast_bits::instructions::SelectInst;



impl BuildFrom for SelectInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> SelectInst {
        let mut item = SelectInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::TypeValue => {
                    item.type_value = TypeValue::build_from(&inner_pair);
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.lhs_type_value = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs_type_value = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}