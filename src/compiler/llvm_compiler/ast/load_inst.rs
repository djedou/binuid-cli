use super::{Type, InstructionItem, AtomicOrdering, Alignment, MetadataAttachment};

#[derive(Debug)]
pub enum LoadInst {
	// Load.
	Atomic1 {
        volatile: bool,
        type_: Type,
        rhs: InstructionItem,
        order: AtomicOrdering,
        align: Alignment,
        metadata_attachments: Vec<MetadataAttachment>
    },
	// Atomic load.
	LoadInst2 {
        volatile: bool,
        type_: Type,
        rhs: InstructionItem,
        order: AtomicOrdering,
        metadata_attachments: Vec<MetadataAttachment>
    },
	LoadInst3 {
        volatile: bool,
        type_: Type,
        rhs: InstructionItem,
        align: Alignment,
        metadata_attachments: Vec<MetadataAttachment>
    },
    LoadInst4 {
        volatile: bool,
        type_: Type,
        rhs: InstructionItem,
        metadata_attachments: Vec<MetadataAttachment>
    },
}