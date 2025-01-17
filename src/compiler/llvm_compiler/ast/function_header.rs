use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::{Visibility, UnnamedAddr, PreemptionSpecifier, DLLStorageClass, CallingConv},
    composes::{FunctionHeader, Section, ReturnAttr, Ident, Param,
    FuncAttr, Comdat},
    types::{TypeConst, Type}
};


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
                /*
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
                },*/
                _ => {}
            }
        }
        
        function_header
    }
}