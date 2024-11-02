use super::Param;


#[derive(Debug)]
pub enum Type {
    Void,
    Func {
        func_type_: Box<FuncType>
    },
    FirstClass {
        first_class_type: Box<FirstClassType>
    }
}


#[derive(Debug)]
pub struct FuncType {
    return_type: Box<Type>,
    params: Vec<Param>
}

#[derive(Debug)]
pub enum FirstClassType { 
    Concrete {
        concrete_type: Box<ConcreteType>
    },
	Metadata
}

#[derive(Debug)]
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


#[derive(Debug)]
pub enum FloatType { 
    Half,
	Float,
	Double,
	X86Dp80,
	Fp128,
	PpcFp128
}

#[derive(Debug)]
pub struct VectorType {
    pub size: u32,
    pub type_: Type
}

#[derive(Debug)]
pub struct ArrayType {
    pub size: u32,
    pub type_: Type
}

#[derive(Debug)]
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