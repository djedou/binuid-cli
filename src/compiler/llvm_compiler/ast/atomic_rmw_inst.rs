use super::{BinOp, TypeValue, AtomicOrdering, MetadataAttachment};

#[derive(Debug)]
pub struct AtomicRMWInst { 
    pub volatile: bool,
    pub bin_op: BinOp,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}