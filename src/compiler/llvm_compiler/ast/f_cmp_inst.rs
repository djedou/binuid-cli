use super::{FastMathFlag, FPred, Type, Value, MetadataAttachment};

#[derive(Debug)]
pub struct FCmpInst { 
    pub flags: Vec<FastMathFlag>,
    pub f_pred: FPred,
    pub type_: Type,
    pub lhs_value: Value,
    pub rhs_value: Value,
    pub metadata_attachments: Vec<MetadataAttachment>
}