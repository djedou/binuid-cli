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

pub struct AtomicStore1 {
    pub volatile: bool,
    pub lhs: InstItem,
    pub rhs: InstItem,
    pub order: AtomicOrdering,
    pub align: Alignment,
    pub metadata_attachments: Vec<MetadataAttachment>
}

pub struct AtomicStore2 {
    pub volatile: bool,
    pub lhs: InstItem,
    pub rhs: InstItem,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}

pub struct VolatileStore1 {
    pub volatile: bool,
    pub lhs: InstItem,
    pub rhs: InstItem,
    pub order: AtomicOrdering,
    pub align: Alignment,
    pub metadata_attachments: Vec<MetadataAttachment>
}

pub struct VolatileStore2 {
    pub volatile: bool,
    pub lhs: InstItem,
    pub rhs: InstItem,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}

pub struct InstItem  {
    pub type_: Type,
    pub value: Value
}