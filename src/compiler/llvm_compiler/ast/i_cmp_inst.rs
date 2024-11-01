use super::{IPred, Type, Value, MetadataAttachment};


pub struct ICmpInst { 
    pub i_pred: IPred,
    pub type_: Type,
    pub lhs_value: Value,
    pub rhs_value: Value,
    pub metadata_attachments: Vec<MetadataAttachment>
}