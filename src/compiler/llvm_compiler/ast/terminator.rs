use super::{
    ConcreteType, Value, MetadataAttachment, Ident, Case,
    CallingConv, ReturnAttr, Arg, OperandBundle, FuncAttr,
    UnwindTarget, Type, ExceptionScope
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Terminator {
    None,
    RetTerm {
        ret: RetTerm
    },
	BrTerm {
        br: BrTerm
    },
	CondBrTerm {
        cond_br: CondBrTerm
    },
	SwitchTerm {
        switch_term: SwitchTerm
    },
	IndirectBrTerm {
        indirect: IndirectBrTerm
    },
	InvokeTerm {
        invoke: InvokeTerm
    },
	ResumeTerm {
        resume: ResumeTerm
    },
	CatchSwitchTerm {
        catch_switch: CatchSwitchTerm
    },
	CatchRetTerm {
        catch_ret: CatchRetTerm
    },
	CleanupRetTerm {
        cleanup: CleanupRetTerm,
    },
	UnreachableTerm {
        unreach: UnreachableTerm
    }
}

#[derive(Debug)]
pub struct UnreachableTerm {
    metadata_attachments: Vec<MetadataAttachment>
}


impl UnreachableTerm {
    pub fn new() -> UnreachableTerm {
        UnreachableTerm {
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct CleanupRetTerm {
    value: Value,
    unwind_target: UnwindTarget,
    metadata_attachments: Vec<MetadataAttachment>
}

impl CleanupRetTerm {
    pub fn new() -> CleanupRetTerm {
        CleanupRetTerm {
            value: Value::None,
            unwind_target: UnwindTarget::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct CatchRetTerm {
    value: Value,
    local_ident: Ident,
    metadata_attachments: Vec<MetadataAttachment>
}

impl CatchRetTerm {
    pub fn new() -> CatchRetTerm {
        CatchRetTerm {
            value: Value::None,
            local_ident: Ident::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct CatchSwitchTerm {
    exception_scope: ExceptionScope,
    label_list: Vec<Ident>,
    unwind_target: UnwindTarget,
    metadata_attachments: Vec<MetadataAttachment>
}

impl CatchSwitchTerm {
    pub fn new() -> CatchSwitchTerm {
        CatchSwitchTerm {
            exception_scope: ExceptionScope::None,
            label_list: vec![],
            unwind_target: UnwindTarget::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct ResumeTerm {
    type_: Type,
    value: Value,
    metadata_attachments: Vec<MetadataAttachment>
}

impl ResumeTerm {
    pub fn new() -> ResumeTerm {
        ResumeTerm {
            type_: Type::None,
            value: Value::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct InvokeTerm {
    calling_conv: CallingConv,
    return_attrs: Vec<ReturnAttr>,
    type_: Type,
    value: Value,
    args: Vec<Arg>,
    func_attrs: Vec<FuncAttr>,
    operand_bundles: Vec<OperandBundle>,
    lhs_local_ident: Ident,
    rhs_local_ident: Ident,
    metadata_attachments: Vec<MetadataAttachment>
}


impl InvokeTerm {
    pub fn new() -> InvokeTerm {
        InvokeTerm {
            calling_conv: CallingConv::None,
            return_attrs: vec![],
            type_: Type::None,
            value: Value::None,
            args: vec![],
            func_attrs: vec![],
            operand_bundles: vec![],
            lhs_local_ident: Ident::None,
            rhs_local_ident: Ident::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct IndirectBrTerm {
    type_: Type,
    value: Value,
    label_list: Vec<Ident>,
    metadata_attachments: Vec<MetadataAttachment>
}


impl IndirectBrTerm {
    pub fn new() -> IndirectBrTerm {
        IndirectBrTerm {
            type_: Type::None,
            value: Value::None,
            label_list: vec![],
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct SwitchTerm {
    type_: Type,
    value: Value,
    local_ident: Ident,
    cases: Vec<Case>,
    metadata_attachments: Vec<MetadataAttachment>
}

impl SwitchTerm {
    pub fn new() -> SwitchTerm {
        SwitchTerm {
            type_: Type::None,
            value: Value::None,
            local_ident: Ident::None,
            cases: vec![],
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct CondBrTerm {
    int_type: u16, 
    value: Value,
    lhs_local_ident: Ident,
    rhs_local_ident: Ident,
    metadata_attachments: Vec<MetadataAttachment>
}

impl CondBrTerm {
    pub fn new() -> CondBrTerm {
        CondBrTerm {
            int_type: 0, 
            value: Value::None,
            lhs_local_ident: Ident::None,
            rhs_local_ident: Ident::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct RetTerm {
    concrete_type: ConcreteType,
    value: Value,
    metadata_attachments: Vec<MetadataAttachment>
}

impl RetTerm {
    pub fn new() -> RetTerm {
        RetTerm {
            concrete_type: ConcreteType::None,
            value: Value::None,
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct BrTerm {
    local_ident: Ident, 
    metadata_attachments: Vec<MetadataAttachment>
}

impl BrTerm {
    pub fn new() -> BrTerm {
        BrTerm {
            local_ident: Ident::None,
            metadata_attachments: vec![]
        }
    }
}


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