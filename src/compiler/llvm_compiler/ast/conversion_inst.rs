use super::{OverflowFlag, ConversionOp, Type, Value, MetadataAttachment};

pub enum ConversionInstFlag {
    OverflowFlags {
        flags: Vec<OverflowFlag>
    },
    Nneg
}

pub struct ConversionInst {
    pub op: ConversionOp,
    pub flag: Option<ConversionInstFlag>,
    pub lhs_type: Type,
    pub value: Value,
    pub rhs_type: Type,
    pub metadata_attachments: Vec<MetadataAttachment>
}
