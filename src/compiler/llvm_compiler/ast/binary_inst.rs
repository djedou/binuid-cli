use super::{BinaryOp, OpFlag, Type, Value, MetadataAttachment};

pub struct BinaryInst { 
    pub op: BinaryOp,
    pub flag: Option<OpFlag>,
    pub type_: Type,
    pub lhs_value: Value,
    pub rhs_value: Value,
    pub metadata_attachments: Vec<MetadataAttachment>
}