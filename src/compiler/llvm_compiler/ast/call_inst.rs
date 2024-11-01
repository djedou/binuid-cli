use super::{FastMathFlag, CallingConv, ReturnAttr, Type, Value, Arg, FuncAttr, OperandBundle, MetadataAttachment};


pub struct CallInst { 
    pub tail: bool,
    pub flags: Vec<FastMathFlag>,
    pub calling_conv: Option<CallingConv>,
    pub return_attrs: Vec<ReturnAttr>,
    pub type_: Type,
    pub value: Value,
    pub args: Vec<Arg>,
    pub func_attrs: Vec<FuncAttr>,
    pub operand_bundles: Vec<OperandBundle>,
    pub metadata_attachments: Vec<MetadataAttachment>
}