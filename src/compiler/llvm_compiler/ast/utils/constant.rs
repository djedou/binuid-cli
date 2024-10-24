
pub enum Constant { 
    Bool {
        value: bool
    },
	Int {
        value: i64
    },
	Float {
        value: FloatLit
    },
	Null,
	None,
	Struct {
        value: StructConst
    },
	Array {
        value: Vec<TypeConst>
    },
	CharArray {
        value: String
    },
	Vector {
        value: Vec<TypeConst>
    },
	ZeroInitializer,
	GlobalIdent {
        ident: Ident
    },
	Undef,
	BlockAddress {
        address: BlockAddressConst
    },
	Expr {
        expr: ConstantExpr
    }
}

pub enum FloatLit { 
    Frac {
        sign: Sign,
        decimals: u32,
        after_dot: u32
    },
	Sci {
        sign: Sign,
        decimals: u32,
        after_dot: u32,
        exp_sign: Sign,
        exp_decimals: u32
    },
	FloatHex {
        value: FloatHexLit
    }
}

pub enum Sign { 
    Plus,
	Minus
}

pub enum FloatHexLit {
    HexFP {
        value: Vec<u8>
    },
    HexFP80 {
        value: Vec<u8>
    },
    HexFP128 {
        value: Vec<u8>
    },
    HexPPC12 {
        value: Vec<u8>
    },
    HexHal {
        value: Vec<u8>
    }
}

pub enum StructConst {
    GrOrLessParath {
        type_list: Vec<TypeConst>
    },
    GrOrLessParathEmpty,
    OnlyParath {
        type_list: Vec<TypeConst>
    },
    OnlyParathEmpty
}


pub struct TypeConst { 
    type_: Type,
    constant: Constant
}

pub struct BlockAddressConst {
    pub global_ident: Ident
    pub local_ident: Ident
}

pub enum ConstantExpr {
	// Binary expressions
    AddExpr {
        flags: Vec<OverflowFlag>,
        lhs: ExprItem,
        rhs: ExprItem
    },
	FAddExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	SubExpr {
        flags: Vec<OverflowFlag>,
        lhs: ExprItem,
        rhs: ExprItem
    },
	FSubExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	MulExpr {
        flags: Vec<OverflowFlag>,
        lhs: ExprItem,
        rhs: ExprItem
    },
	FMulExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	UDivExpr {
        exact: bool,
        lhs: ExprItem,
        rhs: ExprItem
    },
	SDivExpr {
        exact: bool,
        lhs: ExprItem,
        rhs: ExprItem
    },
	FDivExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	URemExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	SRemExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	FRemExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	// Bitwise expressions
	ShlExpr {
        flags: Vec<OverflowFlag>,
        lhs: ExprItem,
        rhs: ExprItem
    },
	LShrExpr {
        exact: bool,
        lhs: ExprItem,
        rhs: ExprItem
    },
	AShrExpr {
        exact: bool,
        lhs: ExprItem,
        rhs: ExprItem
    },
	AndExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	OrExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	XorExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	// Vector expressions
	ExtractElementExpr {
        lhs: ExprItem,
        rhs: ExprItem
    },
	InsertElementExpr {
        lhs: ExprItem,
        rhs: ExprItem,
        item: ExprItem,
    },
	ShuffleVectorExpr {
        lhs: ExprItem,
        rhs: ExprItem,
        item: ExprItem,
    },
	// Aggregate expressions
	ExtractValueExpr {
        lhs: ExprItem,
        indexes: Vec<u32>
    },
	InsertValueExpr {
        lhs: ExprItem,
        rhs: ExprItem,
        indexes: Vec<u32>,
    },
	// Memory expressions
	GetElementPtrExpr {
        lhs: ExprItem,
        rhs: ExprItem,
        indexes: Vec<GEPConstIndex>,
    },
	// Conversion expressions
	TruncExpr {
        lhs: ExprItem,
        type_: Type
    },
	ZExtExpr {
        lhs: ExprItem,
        type_: Type
    },
	SExtExpr {
        lhs: ExprItem,
        type_: Type
    },
	FPTruncExpr {
        lhs: ExprItem,
        type_: Type
    },
	FPExtExpr {
        lhs: ExprItem,
        type_: Type
    },
	FPToUIExpr {
        lhs: ExprItem,
        type_: Type
    },
	FPToSIExpr {
        lhs: ExprItem,
        type_: Type
    },
	UIToFPExpr {
        lhs: ExprItem,
        type_: Type
    },
	SIToFPExpr {
        lhs: ExprItem,
        type_: Type
    },
	PtrToIntExpr {
        lhs: ExprItem,
        type_: Type
    },
	IntToPtrExpr {
        lhs: ExprItem,
        type_: Type
    },
	BitCastExpr {
        lhs: ExprItem,
        type_: Type
    },
	AddrSpaceCastExpr {
        lhs: ExprItem,
        type_: Type
    },
	// Other expressions
	ICmpExpr {
        i_pred: IPred,
        lhs: ExprItem,
        rhs: ExprItem
    },
	FCmpExpr {
        f_pred: FPred
        lhs: ExprItem,
        rhs: ExprItem
    },
	SelectExpr {
        lhs: ExprItem,
        rhs: ExprItem,
        item: ExprItem,
    }
}

pub struct ExprItem {
    pub type_: Type,
    pub constant: Constant
}

pub enum OverflowFlag { 
    Nsw,
	Nuw
}

pub struct GEPConstIndex {
    pub in_range: bool,
    pub rhs: ExprItem
}

pub enum IPred { 
    Eq,
	Ne,
	Sge,
	Sgt,
	Sle,
	Slt,
	Uge,
	Ugt,
	Ule,
	Ult
}

pub enum FPred { 
    False,
	True,
    Oeq,
    Oge,
    Ogt,
    Ole,
    Olt,
    One,
    Ord,
    Ueq,
    Uge,
    Ugt,
    Ule,
    Ult,
    Une,
    Uno
}