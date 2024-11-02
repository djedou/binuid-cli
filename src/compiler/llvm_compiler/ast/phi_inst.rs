use super::{Type, Inc, MetadataAttachment};

#[derive(Debug)]
pub struct PhiInst {
    pub type_: Type,
    pub inc_list: Vec<Inc>,
    pub metadata_attachments: Vec<MetadataAttachment>
}