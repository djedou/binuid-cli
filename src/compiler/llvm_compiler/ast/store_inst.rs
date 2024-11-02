use super::{TypeValue, AtomicOrdering, Alignment, MetadataAttachment};

#[derive(Debug)]
pub enum StoreInst { 
    Atomic1 {
        value: AtomicStore1
    },
	Atomic2 {
        value: AtomicStore2
    },
	Volatile1 {
        value: VolatileStore1
    },
	Volatile2 {
        value: VolatileStore2
    }
}


#[derive(Debug)]
pub struct AtomicStore1 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub align: Alignment,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AtomicStore2 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct VolatileStore1 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub align: Alignment,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct VolatileStore2 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}