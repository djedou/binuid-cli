use super::{Type, Value, MetadataAttachment};

pub struct VAArgInst { 
    pub lhs_type:Type,
    pub value: Value,
    pub rhs_type: Type,
    pub metadata_attachments: Vec<MetadataAttachment>
}