use super::{ExceptionScope, ExceptionArg, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct CleanupPadInst { 
    pub exception_scope: ExceptionScope,
    pub exception_args: Vec<ExceptionArg>,
    pub metadata_attachments: Vec<MetadataAttachment>
}


impl CleanupPadInst {
    pub fn new() -> CleanupPadInst {
        CleanupPadInst {
            exception_scope: ExceptionScope::None,
            exception_args: vec![],
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for CleanupPadInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CleanupPadInst {
        let mut item = CleanupPadInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ExceptionScope => {
                    item.exception_scope = ExceptionScope::build_from(&inner_pair);
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