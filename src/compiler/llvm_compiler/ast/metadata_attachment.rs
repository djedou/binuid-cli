use super::{MetadataName, MDNode};


#[derive(Debug)]
pub struct MetadataAttachment {
    pub metadata_name: MetadataName,
    pub md_node: MDNode
}