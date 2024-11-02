use super::{TypeValue, MetadataAttachment};


#[derive(Debug)]
pub struct SelectInst { 
    pub lhs_type_value: TypeValue,
    pub rhs_type_value: TypeValue,
    pub type_value: TypeValue,
    pub metadata_attachments: Vec<MetadataAttachment>
}