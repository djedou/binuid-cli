use super::{FunctionHeader, MetadataAttachment};


#[derive(Debug)]
pub struct FunctionDecl {
    pub metadata_attachments: Vec<MetadataAttachment>,
    pub extern_linkage: bool,
    pub function_header: FunctionHeader
}