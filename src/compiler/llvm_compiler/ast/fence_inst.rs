use super::{MetadataAttachment, AtomicOrdering};


#[derive(Debug)]
pub struct FenceInst { 
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}