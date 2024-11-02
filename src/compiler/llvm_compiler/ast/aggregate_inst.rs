use super::{AggregateOp, TypeValue, Index, MetadataAttachment};

#[derive(Debug)]
pub struct AggregateInst {
    pub op: AggregateOp,
    pub lhs_type_value: TypeValue,
    pub rhs_type_value: Option<TypeValue>,
    pub indices: Vec<Index>,
    pub metadata_attachments: Vec<MetadataAttachment>
}