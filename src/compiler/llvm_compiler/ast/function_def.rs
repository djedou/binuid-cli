use super::{FunctionHeader, FunctionBody, MetadataAttachment};

pub struct FunctionDef {
    pub linkage: bool,
    pub function_header: FunctionHeader,
    pub metadata_attachments: Vec<MetadataAttachment>,
    pub function_body: FunctionBody
}