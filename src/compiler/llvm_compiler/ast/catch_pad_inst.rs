use super::{Ident, ExceptionArg, MetadataAttachment};


#[derive(Debug)]
pub struct CatchPadInst { 
    pub local_ident: Ident,
    pub exception_args: Vec<ExceptionArg>,
    pub metadata_attachments: Vec<MetadataAttachment>
}