mod type_def;

pub use type_def::*;

pub enum Type {
    Void,
    Func {
        func_type_: FuncType
    },
    FirstClass {
        first_class_type: FirstClassType
    }
}

pub struct FuncType {
    return_type: Box<Type>,
    params: Params
}

pub enum FirstClassType { 
    Concrete {
        concrete_type: ConcreteType
    },
	Metadata
}

pub enum ConcreteType { 
    Int {
        decimals: u16
    },
	Float {
        float_type: FloatType
    },
	Ptr,
	Vec {
        vec: VectorType
    },
	Label,
	Array {
        array: ArrayType
    },
	Struct {
        struct_type: StructType
    },
	Named {
        ident: String
    },
	MMX,
	Token,
    Void
}

pub enum FloatType { 
    Half,
	Float,
	Double,
	X86Dp80,
	Fp128,
	PpcFp128
}

pub struct VectorType {
    pub size: u32,
    pub type_: Type
}

pub struct ArrayType {
    pub size: u32,
    pub type_: Type
}

pub enum StructType {
    GrOrLessParath {
        type_list: Vec<Type>
    },
    GrOrLessParathEmpty,
    OnlyParath {
        type_list: Vec<Type>
    },
    OnlyParathEmpty
}