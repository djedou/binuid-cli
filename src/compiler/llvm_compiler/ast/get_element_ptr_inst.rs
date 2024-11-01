use super::{MemoryOp, Type, Value, TypeValue, MetadataAttachment};



pub struct GetElementPtrInst {
    pub op: MemoryOp,
    pub in_bounds: bool,
    pub lhs_type: Type,
    pub rhs_type: Type,
    pub value: Value,
    pub type_values: Vec<TypeValue>,
    pub metadata_attachments: Vec<MetadataAttachment>
}