use super::{Param, Ident};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Type {
    None,
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
    None,
    Concrete {
        concrete_type: Box<ConcreteType>
    },
	Metadata
}

#[derive(Debug)]
pub enum ConcreteType {
    None, 
    Int {
        bits: u16
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
        ident: Ident
    },
	MMX,
	Token,
    Void
}


#[derive(Debug)]
pub enum FloatType {
    None,
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
    None,
    GrOrLessParath {
        type_list: Vec<Type>
    },
    GrOrLessParathEmpty,
    OnlyParath {
        type_list: Vec<Type>
    },
    OnlyParathEmpty
}

impl BuildFrom for Type {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Type {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::VoidType => Type::Void,
                    Rule::FuncType => Type::Func {
                        func_type_: Box::new(FuncType::build_from(&inner_pair))
                    },
                    Rule::FirstClassType => Type::FirstClass {
                        first_class_type: Box::new(FirstClassType::build_from(&inner_pair))
                    },
                    _ => Type::None
                }
            },
            None => Type::None
        }
    }
}

impl BuildFrom for FirstClassType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FirstClassType {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::MetadataType => FirstClassType::Metadata,
                    Rule::ConcreteType => FirstClassType::Concrete {
                        concrete_type: Box::new(ConcreteType::build_from(&inner_pair))
                    },
                    _ => FirstClassType::None
                }
            },
            None => FirstClassType::None
        }
    }
}


impl BuildFrom for FuncType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FuncType {
        let mut func_type = FuncType {
            return_type: Box::new(Type::None),
            params: vec![]
        };
        
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::NoFuncType => {
                    func_type.return_type = Box::new(Type::build_from(&inner_pair));
                },
                Rule::Params => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Param => {
                                func_type.params.push(Param::build_from(&p));
                            },
                            Rule::DotDotDot => {
                                func_type.params.push(Param::new_dot_dot_dot());
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        func_type
    }
}

impl BuildFrom for ConcreteType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ConcreteType {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::IntType => {
                        match inner_pair.clone().into_inner().next() {
                            Some(p) => {
                                match p.as_rule() {
                                    Rule::Decimals => p.as_str().parse::<u16>().map_or(ConcreteType::None, |d|  ConcreteType::Int {
                                        bits: d.clone()
                                    }),
                                    _ => ConcreteType::None
                                }
                            },
                            None => ConcreteType::None
                        }
                    },
                    Rule::FloatType => ConcreteType::Float {
                        float_type: FloatType::build_from(&inner_pair)
                    },
                    Rule::PointerPtrType => ConcreteType::Ptr,
                    Rule::VectorType => ConcreteType::Vec {
                        vec: VectorType::build_from(&inner_pair)
                    },
                    Rule::ArrayType => ConcreteType::Array {
                        array: ArrayType::build_from(&inner_pair)
                    },
                    Rule::LabelType => ConcreteType::Label,
                    Rule::StructType => ConcreteType::Struct {
                        struct_type: StructType::build_from(&inner_pair)
                    },
                    Rule::NamedType => {
                        match inner_pair.clone().into_inner().next() {
                            Some(p) => {
                                match p.as_rule() {
                                    Rule::LocalIdent => ConcreteType::Named {
                                        ident: Ident::build_from(&p)
                                    },
                                    _ => ConcreteType::None
                                }
                            },
                            None => ConcreteType::None
                        }
                    },
                    Rule::MMXType => ConcreteType::MMX,
                    Rule::TokenType => ConcreteType::Token,
                    Rule::VoidType => ConcreteType::Void,
                    _ => ConcreteType::None
                }
            },
            None => ConcreteType::None
        }
    }
}

impl BuildFrom for FloatType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FloatType {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Half => FloatType::Half,
                    Rule::Half => FloatType::Half,
                    Rule::Double => FloatType::Double,
                    Rule::X86Dp80 => FloatType::X86Dp80,
                    Rule::Fp128 => FloatType::Fp128,
                    Rule::PpcFp128 => FloatType::PpcFp128,
                    _ => FloatType::None
                }
            },
            None => FloatType::None
        }
    }
}

impl BuildFrom for VectorType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VectorType {
        let mut vector_type = VectorType {
            size: 0,
            type_: Type::None
        };
        
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::IntLit => {
                    vector_type.size = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                Rule::Type => {
                    vector_type.type_ = Type::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        vector_type
    }
}

impl BuildFrom for ArrayType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ArrayType {
        let mut array_type = ArrayType {
            size: 0,
            type_: Type::None
        };
        
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::IntLit => {
                    array_type.size = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                Rule::Type => {
                    array_type.type_ = Type::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        array_type
    }
}

impl BuildFrom for StructType {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> StructType {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::GrOrLessParathType => {
                        let mut type_list = vec![];
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::Type => {
                                    type_list.push(Type::build_from(&p))
                                },
                                _ => {}
                            }
                        }

                        StructType::GrOrLessParath {
                            type_list: type_list
                        }
                    },
                    Rule::GrOrLessParathEmptyType => StructType::GrOrLessParathEmpty,
                    Rule::OnlyParathType => {
                        let mut type_list = vec![];
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::Type => {
                                    type_list.push(Type::build_from(&p))
                                },
                                _ => {}
                            }
                        }

                        StructType::OnlyParath {
                            type_list: type_list
                        }
                    },
                    Rule::OnlyParathEmptyType => StructType::OnlyParathEmpty,
                    _ => StructType::None
                }
            },
            None => StructType::None
        }
    }
}