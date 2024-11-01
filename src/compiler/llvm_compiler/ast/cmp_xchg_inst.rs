use super::{TypeValue, AtomicOrdering, MetadataAttachment};

pub struct CmpXchgInst { 
    pub weak: bool,
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub type_value: TypeValue,
    pub order_1: AtomicOrdering,
    pub order_2: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}