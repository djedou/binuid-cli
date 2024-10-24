
pub enum AllocaInst { 
    AllocaInst1 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        rhs: InstItem,
        align: Alignment,
        addr_space: u32,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst2 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        rhs: InstItem,
        addr_space: u32,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst3 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        rhs: InstItem,
        align: Alignment,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst4 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        align: Alignment,
        addr_space: u32,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst5 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        rhs: InstItem,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst6 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        align: Alignment,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst7 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        addr_space: u32,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AllocaInst8 {
        in_alloca: bool,
        swift_error: bool,
        type_: Type,
        metadata_attachments: Vec<MetadataAttachment>
    }
}