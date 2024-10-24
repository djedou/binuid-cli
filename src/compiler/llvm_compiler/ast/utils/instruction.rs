pub struct InstructionItem {
    start_comment: Option<String>,
    instruction: Instruction,
    end_comment: Option<String>
}

pub enum Instruction {
	// Instructions? not producing values.
    Store {
        inst: StoreInst
    },
	Fence {
        order: AtomicOrdering,
        attachments: Vec<MetadataAttachment>
    },
	CmpXchg {
        weak: bool,
        volatile: bool,
        lhs: InstItem,
        rhs: InstItem,
        type_: Type,
        item: InstItem,
        order1: AtomicOrdering,
        order2: AtomicOrdering,
        attachments: Vec<MetadataAttachment>
    },
	AtomicRMWInst {
        volatile: bool,
        bin_op: BinOp,
        lhs: InstItem,
        rhs: InstItem,
        order: AtomicOrdering,
        attachments: Vec<MetadataAttachment>
    },
	// Instructions? producing values.
	LocalIdent {
        ident: Ident,
        value_inst: ValueInstruction
    },
    Value {
        inst: ValueInstruction
    }
}

pub enum BinOp { 
    Add,
    And,
    Max,
    Min,
    Nand,
    Or,
    Sub,
    Umax,
    Umin,
    Xchg,
    Xor
}

pub enum ValueInstruction {
	// Binary instructions
    Add {
        overflow_flags: Vec<OverflowFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	FAdd {
        fast_math_flags: Vec<FastMathFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	Sub {
        overflow_flags: Vec<OverflowFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	FSub {
        fast_math_flags: Vec<FastMathFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	Mul {
        overflow_flags: Vec<OverflowFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	FMul {
        fast_math_flags: Vec<FastMathFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	UDiv {
        exact: bool,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	SDiv {
        exact: bool,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	FDiv {
        fast_math_flags: Vec<FastMathFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	URem {
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	SRem {
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	FRem {
        fast_math_flags: Vec<FastMathFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	// Bitwise instructions?
	Shl {
        overflow_flags: Vec<OverflowFlag>,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	LShr {
        exact: bool,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	AShr {
        exact: bool,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	And {
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	Or {
        disjoint: bool,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	Xor {
        disjoint: bool,
        lhs: InstItem,
        value: Value,
        metadata_attachments: Vec<MetadataAttachment>
    },
	// Vector instructions?
	ExtractElement {
        lhs: InstItem,
        rhs: InstItem,
        metadata_attachments: Vec<MetadataAttachment>
    },
	InsertElement {
        lhs: InstItem,
        rhs: InstItem,
        item: InstItem,
        metadata_attachments: Vec<MetadataAttachment>
    },
	ShuffleVector {
        lhs: InstItem,
        rhs: InstItem,
        item: InstItem,
        metadata_attachments: Vec<MetadataAttachment>
    },
	// Aggregate instructions?
	ExtractValue {
        lhs: InstItem,
        indexes: Vec<u32>,
        metadata_attachments: Vec<MetadataAttachment>
    },
	InsertValue {
        lhs: InstItem,
        rhs: InstItem,
        indexes: Vec<u32>,
        metadata_attachments: Vec<MetadataAttachment>
    },
	// Memory instructions?
	Alloca {
        inst: AllocaInst
    },
	Load {
        inst: LoadInst
    },
	GetElementPtrInst,
	// Conversion instructions?
	ZExtInst,
	SExtInst,
	FPTruncInst,
	FPExtInst,
	FPToUIInst,
	FPToSIInst,
	TruncInst,
	UIToFPInst,
	SIToFPInst,
	PtrToIntInst,
	IntToPtrInst,
	BitCastInst,
	AddrSpaceCastInst,
	// Other instructions?
	ICmpInst,
	FCmpInst,
	PhiInst,
	SelectInst,
	CallInst,
	VAArgInst,
	LandingPadInst,
	CatchPadInst,
	CleanupPadInst,
    Terminator
}