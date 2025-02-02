use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{ExceptionArg, Ident, MetadataAttachment};
use binuid_shared_wasm::ast_bits::instructions::CatchPadInst;






impl BuildFrom for CatchPadInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CatchPadInst {
        let mut item = CatchPadInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::LocalIdent => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::ExceptionArgs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::ExceptionArg => {
                                item.exception_args.push(ExceptionArg::build_from(&p));
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