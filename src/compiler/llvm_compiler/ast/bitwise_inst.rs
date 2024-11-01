use super::{BitwiseOp, OpFlag, Type, Value, MetadataAttachment};

pub struct BitwiseInst { 
    pub op: BitwiseOp,
    pub flag: Option<OpFlag>,
    pub type_: Type,
    pub lhs_value: Value,
    pub rhs_value: Value,
    pub metadata_attachments: Vec<MetadataAttachment>
}