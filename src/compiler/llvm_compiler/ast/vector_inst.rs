use super::{VectorOp, TypeValue, MetadataAttachment};

pub struct VectorInst { 
    pub op: VectorOp,
    pub lhs_type_value: TypeValue,
    pub rhs_type_value: TypeValue,
    pub type_value: Option<TypeValue>,
    pub metadata_attachments: Vec<MetadataAttachment>
}