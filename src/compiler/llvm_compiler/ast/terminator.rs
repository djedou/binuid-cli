use super::{
    ConcreteType, Value, MetadataAttachment, Ident, Case,
    CallingConv, ReturnAttr, Arg, OperandBundle, FuncAttr,
    UnwindTarget, Type, ExceptionScope
};


pub enum Terminator { 
    RetTerm {
        concrete_type: ConcreteType,
        value: Option<Value>,
        metadata_attachments: Vec<MetadataAttachment>
    },
	BrTerm {
        local_ident: Ident, 
        metadata_attachments: Vec<MetadataAttachment>
    },
	CondBrTerm {
        int_type: u32, 
        value: Value,
        lhs_local_ident: Ident,
        rhs_local_ident: Ident,
        metadata_attachments: Vec<MetadataAttachment>
    },
	SwitchTerm {
        type_: Type,
        value: Value,
        local_ident: Ident,
        cases: Vec<Case>,
        metadata_attachments: Vec<MetadataAttachment>
    },
	IndirectBrTerm {
        type_: Type,
        value: Value,
        label_list: Vec<Ident>,
        metadata_attachments: Vec<MetadataAttachment>
    },
	InvokeTerm {
        calling_conv: Option<CallingConv>,
        return_attrs: Vec<ReturnAttr>,
        type_: Type,
        value: Value,
        args: Vec<Arg>,
        func_attrs: Vec<FuncAttr>,
        operand_bundles: Vec<OperandBundle>,
        lhs_local_ident: Ident,
        rhs_local_ident:Ident,
        metadata_attachments: Vec<MetadataAttachment>
    },
	ResumeTerm {
        type_: Type,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	CatchSwitchTerm {
        exception_scope: ExceptionScope,
        label_list: Vec<Ident>,
        unwind_target: UnwindTarget,
        metadata_attachments: Vec<MetadataAttachment>
    },
	CatchRetTerm {
        value: Value,
        local_ident: Ident,
        metadata_attachments: Vec<MetadataAttachment>
    },
	CleanupRetTerm {
        value: Value,
        unwind_target: UnwindTarget,
        metadata_attachments: Vec<MetadataAttachment>
    },
	UnreachableTerm {
        metadata_attachments: Vec<MetadataAttachment>
    }
}