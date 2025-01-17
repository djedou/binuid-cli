use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::CallingConv,
    composes::{Ident, Case, ReturnAttr, Terminator, UnreachableTerm, CleanupRetTerm, CatchRetTerm,
        ResumeTerm, InvokeTerm, CatchSwitchTerm, IndirectBrTerm, ExceptionScope, Arg, FuncAttr,
        SwitchTerm, CondBrTerm, BrTerm, RetTerm, MetadataAttachment, UnwindTarget, OperandBundle
    }
};
use binuid_shared_wasm::ast_bits::types::{Type, ConcreteType};
use binuid_shared_wasm::ast_bits::values::Value;


impl BuildFrom for UnreachableTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> UnreachableTerm {
        let mut item = UnreachableTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}



impl BuildFrom for CleanupRetTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CleanupRetTerm {
        let mut item = CleanupRetTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::UnwindTarget => {
                    item.unwind_target = UnwindTarget::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for CatchRetTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CatchRetTerm {
        let mut item = CatchRetTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::LocalIdent => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for CatchSwitchTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CatchSwitchTerm {
        let mut item = CatchSwitchTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ExceptionScope => {
                    item.exception_scope = ExceptionScope::build_from(&inner_pair);
                },
                Rule::LabelList => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::LocalIdent => {
                                item.label_list.push(Ident::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::UnwindTarget => {
                    item.unwind_target = UnwindTarget::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for ResumeTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ResumeTerm {
        let mut item = ResumeTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for InvokeTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> InvokeTerm {
        let mut item = InvokeTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::CallingConv => {
                    item.calling_conv = CallingConv::build_from(&inner_pair);
                },
                Rule::ReturnAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::ReturnAttr => {
                                item.return_attrs.push(ReturnAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::Args => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Arg => {
                                item.args.push(Arg::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::FuncAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::FuncAttr => {
                                item.func_attrs.push(FuncAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::OperandBundles => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::OperandBundle => {
                                item.operand_bundles.push(OperandBundle::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::LhsLocalIdent => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::LocalIdent => {
                                item.lhs_local_ident = Ident::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsLocalIdent => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::LocalIdent => {
                                item.rhs_local_ident = Ident::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for IndirectBrTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> IndirectBrTerm {
        let mut item = IndirectBrTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::LabelList => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::LocalIdent => {
                                item.label_list.push(Ident::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for SwitchTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> SwitchTerm {
        let mut item = SwitchTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::LocalIdent => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::Cases => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Case => {
                                item.cases.push(Case::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}

impl BuildFrom for CondBrTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CondBrTerm {
        let mut item = CondBrTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::IntType => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::Decimals => {
                                    item.int_type = p.as_str().parse::<u16>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        },
                        None => {}
                    }
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::LhsLocalIdent => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::LocalIdent => {
                                item.lhs_local_ident = Ident::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsLocalIdent => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::LocalIdent => {
                                item.rhs_local_ident = Ident::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for BrTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BrTerm {
        let mut item = BrTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::LocalIdent => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}



impl BuildFrom for RetTerm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> RetTerm {
        let mut item = RetTerm::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ConcreteType => {
                    item.concrete_type = ConcreteType::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}

impl BuildFrom for Terminator {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Terminator {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::RetTerm => Terminator::RetTerm {
                        ret: RetTerm::build_from(&inner_pair)
                    },
                    Rule::BrTerm => Terminator::BrTerm {
                        br: BrTerm::build_from(&inner_pair)
                    },
                    Rule::CondBrTerm => Terminator::CondBrTerm {
                        cond_br: CondBrTerm::build_from(&inner_pair)
                    },
                    Rule::SwitchTerm => Terminator::SwitchTerm {
                        switch_term: SwitchTerm::build_from(&inner_pair)
                    },
                    Rule::IndirectBrTerm => Terminator::IndirectBrTerm {
                        indirect: IndirectBrTerm::build_from(&inner_pair)
                    },
                    Rule::InvokeTerm => Terminator::InvokeTerm {
                        invoke: InvokeTerm::build_from(&inner_pair)
                    },
                    Rule::ResumeTerm => Terminator::ResumeTerm {
                        resume: ResumeTerm::build_from(&inner_pair)
                    },
                    Rule::CatchSwitchTerm => Terminator::CatchSwitchTerm {
                        catch_switch: CatchSwitchTerm::build_from(&inner_pair)
                    },
                    Rule::CatchRetTerm => Terminator::CatchRetTerm {
                        catch_ret: CatchRetTerm::build_from(&inner_pair)
                    },
                    Rule::CleanupRetTerm => Terminator::CleanupRetTerm {
                        cleanup: CleanupRetTerm::build_from(&inner_pair)
                    },
                    Rule::UnreachableTerm => Terminator::UnreachableTerm {
                        unreach: UnreachableTerm::build_from(&inner_pair)
                    },
                    _ => Terminator::None
                }
            },
            None => Terminator::None
        }
    }
}