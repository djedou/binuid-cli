use super::{MetadataAttachment, AtomicOrdering};

pub struct FenceInst { 
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}