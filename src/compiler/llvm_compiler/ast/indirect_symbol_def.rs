use super::{
    Ident, PreemptionSpecifier, Visibility, DLLStorageClass, ThreadLocal, UnnamedAddr,
    Alias, Type, TypeConst
};


#[derive(Debug)]
pub struct IndirectSymbolDef {
    pub global_ident: Ident,
    pub linkage_kind: LinkageKind,
    pub preemption_specifier: Option<PreemptionSpecifier>,
    pub visibility: Option<Visibility>,
    pub dll_storage_class: Option<DLLStorageClass>,
    pub thread_local: Option<ThreadLocal>,
    pub unnamed_addr: Option<UnnamedAddr>,
    pub alias: Alias,
    pub type_: Type,
    pub type_const: TypeConst
}


#[derive(Debug)]
pub enum LinkageKind { 
    ExternLinkage,
    Linkage
}