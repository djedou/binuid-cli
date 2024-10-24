


pub enum LoadInst {
	// Load.
	Atomic1 {
        volatile: bool,
        type_: Type,
        rhs: InstItem,
        order: AtomicOrdering,
        align: Alignment,
        metadata_attachments: Vec<MetadataAttachment>
    },
	// Atomic load.
	LoadInst2 {
        volatile: bool,
        type_: Type,
        rhs: InstItem,
        order: AtomicOrdering,
        metadata_attachments: Vec<MetadataAttachment>
    },
	LoadInst3 {
        volatile: bool,
        type_: Type,
        rhs: InstItem,
        align: Alignment,
        metadata_attachments: Vec<MetadataAttachment>
    },
    LoadInst4 {
        volatile: bool,
        type_: Type,
        rhs: InstItem,
        metadata_attachments: Vec<MetadataAttachment>
    },
}