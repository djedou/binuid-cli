use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{MetadataAttachment, FunctionHeader};


#[derive(Debug)]
pub struct FunctionDecl {
    pub metadata_attachments: Vec<MetadataAttachment>,
    pub extern_linkage: bool,
    pub function_header: FunctionHeader
}

impl BuildFrom for FunctionDecl {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FunctionDecl {
        let mut function_decl = FunctionDecl {
            metadata_attachments: vec![],
            extern_linkage: false,
            function_header: FunctionHeader::new()
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MetadataAttachment => {
                    function_decl.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                Rule::ExternLinkage => {
                    function_decl.extern_linkage = true;
                },
                Rule::FunctionHeader => {
                    function_decl.function_header = FunctionHeader::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        function_decl
    }
}