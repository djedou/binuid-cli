use super::{ExceptionScope, ExceptionArg, MetadataAttachment};



#[derive(Debug)]
pub struct CleanupPadInst { 
    pub exception_scope: ExceptionScope,
    pub exception_args: Vec<ExceptionArg>,
    pub metadata_attachments: Vec<MetadataAttachment>
}