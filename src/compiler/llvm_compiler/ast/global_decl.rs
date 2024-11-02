use super::{ Type, Ident, ExternLinkage, PreemptionSpecifier, Visibility, DLLStorageClass, 
    ThreadLocal, UnnamedAddr, Immutable, GlobalAttr, FuncAttr
};


#[derive(Debug)]
pub struct GlobalDecl {
    pub ident: Ident,
    pub extern_linkage: ExternLinkage,
    pub preemption_specifier: Option<PreemptionSpecifier>,
    pub visibility: Option<Visibility>,
    pub dll_storage_class: Option<DLLStorageClass>,
    pub thread_local: Option<ThreadLocal>,
    pub unnamed_addr: Option<UnnamedAddr>,
    pub addr_space: Option<u32>,
    pub externally_initialized: bool,
    pub immutable: Immutable,
    pub type_: Type,
    pub global_attrs: Vec<GlobalAttr>,
    pub func_attrs: Vec<FuncAttr>
}