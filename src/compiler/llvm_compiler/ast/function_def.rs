use super::{FunctionBody, MetadataAttachment, FunctionHeader};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct FunctionDef {
    pub linkage: bool,
    pub function_header: FunctionHeader,
    pub metadata_attachments: Vec<MetadataAttachment>,
    pub function_body: FunctionBody
}


impl FunctionDef {
    pub fn new() -> FunctionDef {
        FunctionDef {
            linkage: false,
            function_header: FunctionHeader::new(),
            metadata_attachments: vec![],
            function_body: FunctionBody::new()
        }
    }
}

impl BuildFrom for FunctionDef {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FunctionDef {
        let mut function_def = FunctionDef::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Linkage => {
                    function_def.linkage = true;
                },
                Rule::FunctionHeader => {
                    function_def.function_header = FunctionHeader::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    function_def.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                Rule::FunctionBody => {
                    function_def.function_body = FunctionBody::build_from(&inner_pair);
                },
                _ => {}
            }
        }
        
        function_def
    }
}