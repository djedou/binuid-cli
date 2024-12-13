use super::{
    Ident, PreemptionSpecifier, Visibility, DLLStorageClass, ThreadLocal, UnnamedAddr,
    Alias, Type, TypeConst
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};



#[derive(Debug)]
pub struct IndirectSymbolDef {
    pub global_ident: Ident,
    pub linkage_kind: LinkageKind,
    pub preemption_specifier: PreemptionSpecifier,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass,
    pub thread_local: ThreadLocal,
    pub unnamed_addr: UnnamedAddr,
    pub alias: Alias,
    pub type_: Type,
    pub type_const: TypeConst
}


#[derive(Debug)]
pub enum LinkageKind {
    None,
    ExternLinkage,
    Linkage
}



impl BuildFrom for IndirectSymbolDef {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> IndirectSymbolDef {
        let mut symbol_def = IndirectSymbolDef {
            global_ident: Ident::None,
            linkage_kind: LinkageKind::None,
            preemption_specifier: PreemptionSpecifier::None,
            visibility: Visibility::None,
            dll_storage_class: DLLStorageClass::None,
            thread_local: ThreadLocal::new(),
            unnamed_addr: UnnamedAddr::None,
            alias: Alias::None,
            type_: Type::None,
            type_const: TypeConst::new()
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::GlobalIdent => {
                    symbol_def.global_ident = Ident::build_from(&inner_pair);
                },
                Rule::LinkageKind => {
                    symbol_def.linkage_kind = LinkageKind::build_from(&inner_pair);
                },
                Rule::PreemptionSpecifier => {
                    symbol_def.preemption_specifier = PreemptionSpecifier::build_from(&inner_pair);
                },
                Rule::Visibility => {
                    symbol_def.visibility = Visibility::build_from(&inner_pair);
                },
                Rule::DLLStorageClass => {
                    symbol_def.dll_storage_class = DLLStorageClass::build_from(&inner_pair);
                },
                Rule::ThreadLocal => {
                    symbol_def.thread_local = ThreadLocal::build_from(&inner_pair);
                },
                Rule::UnnamedAddr => {
                    symbol_def.unnamed_addr = UnnamedAddr::build_from(&inner_pair);
                },
                Rule::Alias => {
                    symbol_def.alias = Alias::build_from(&inner_pair);
                },
                Rule::Type => {
                    symbol_def.type_ = Type::build_from(&inner_pair);
                },
                Rule::TypeConst => {
                    symbol_def.type_const = TypeConst::build_from(&inner_pair);
                },
                _ => {}
            }
        }
        
        symbol_def
    }
}

impl BuildFrom for LinkageKind {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LinkageKind {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ExternLinkage => LinkageKind::ExternLinkage,
                    Rule::Linkage => LinkageKind::Linkage,
                    _ => LinkageKind::None
                }
            },
            None => LinkageKind::None
        }
    }
}