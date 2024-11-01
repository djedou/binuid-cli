use super::{ExceptionScope, ExceptionArg, MetadataAttachment};

pub struct CleanupPadInst { 
    pub exception_scope: ExceptionScope,
    pub exception_args: Vec<ExceptionArg>,
    pub metadata_attachments: Vec<MetadataAttachment>
}