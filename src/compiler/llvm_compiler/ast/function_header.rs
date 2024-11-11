use super::{
    PreemptionSpecifier, Visibility, DLLStorageClass, CallingConv, ReturnAttr, Type, Ident, Param, UnnamedAddr,
    FuncAttr, TypeConst, Section, Comdat
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct FunctionHeader {
    pub preemption_specifier: PreemptionSpecifier,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass,
    pub calling_conv: CallingConv,
    pub return_attrs: Vec<ReturnAttr>,
    pub type_: Type,
    pub global_ident: Ident,
    pub params: Vec<Param>,
    pub unnamed_addr: UnnamedAddr,
    pub func_attrs: Vec<FuncAttr>,
    pub section: Section,
    pub comdat: Comdat,
    pub g: String,
    pub prefix: TypeConst,
    pub prologue: TypeConst,
    pub personality: TypeConst
}


impl FunctionHeader {
    pub fn new() -> FunctionHeader {
        FunctionHeader {
            preemption_specifier: PreemptionSpecifier::None,
            visibility: Visibility::None,
            dll_storage_class: DLLStorageClass::None,
            calling_conv: CallingConv::None,
            return_attrs: vec![],
            type_: Type::None,
            global_ident: Ident::None,
            params: vec![],
            unnamed_addr: UnnamedAddr::None,
            func_attrs: vec![],
            section: Section::new(),
            comdat: Comdat::None,
            g: String::with_capacity(0),
            prefix: TypeConst::new(),
            prologue: TypeConst::new(),
            personality: TypeConst::new()
        }
    }
}



impl BuildFrom for FunctionHeader {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FunctionHeader {
        let mut function_header = FunctionHeader::new();
        
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::PreemptionSpecifier => {
                    function_header.preemption_specifier = PreemptionSpecifier::build_from(&inner_pair);
                },
                Rule::Visibility => {
                    function_header.visibility = Visibility::build_from(&inner_pair);
                },
                Rule::DLLStorageClass => {
                    function_header.dll_storage_class = DLLStorageClass::build_from(&inner_pair);
                },
                Rule::CallingConv => {
                    function_header.calling_conv = CallingConv::build_from(&inner_pair);
                },
                Rule::ReturnAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::ReturnAttr => {
                                function_header.return_attrs.push(ReturnAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Type => {
                    function_header.type_ = Type::build_from(&inner_pair);
                },
                Rule::GlobalIdent => {
                    function_header.global_ident = Ident::build_from(&inner_pair);
                },
                Rule::Params => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Param => {
                                function_header.params.push(Param::build_from(&p));
                            },
                            Rule::DotDotDot => {
                                function_header.params.push(Param::new_dot_dot_dot());
                            },
                            _ => {}
                        }
                    }
                },
                Rule::UnnamedAddr => {
                    function_header.unnamed_addr = UnnamedAddr::build_from(&inner_pair);
                },
                Rule::FuncAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::FuncAttr => {
                                function_header.func_attrs.push(FuncAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Section => {
                    function_header.section = Section::build_from(&inner_pair);
                },
                Rule::Comdat => {
                    function_header.comdat = Comdat::build_from(&inner_pair);
                },
                Rule::OptGC => {
                    function_header.g = String::build_from(&inner_pair);
                },
                Rule::OptPrefix => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeConst => {
                                function_header.prefix = TypeConst::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::OptPrologue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeConst => {
                                function_header.prologue = TypeConst::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::OptPersonality => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeConst => {
                                function_header.personality = TypeConst::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
        
        function_header
    }
}