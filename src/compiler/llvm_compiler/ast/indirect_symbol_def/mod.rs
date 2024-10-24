

pub enum IndirectSymbolDef {
    Extern {
        global_ident: Ident,
        extern_linkage: ExternLinkage,
        preemption_specifier: Option<PreemptionSpecifier>,
        visibility: Option<Visibility>,
        dll_storage_class: Option<DLLStorageClass>,
        thread_local: Option<ThreadLocal>,
        unnamed_addr: Option<UnnamedAddr>,
        Alias: Alias,
        type_: Type,
        expr: ExprItem
    },
    Opt {
        global_ident: Ident,
        linkage: Linkage,
        preemption_specifier: Option<PreemptionSpecifier>,
        visibility: Option<Visibility>,
        dll_storage_class: Option<DLLStorageClass>,
        thread_local: Option<ThreadLocal>,
        unnamed_addr: Option<UnnamedAddr>,
        Alias: Alias,
        type_: Type,
        expr: ExprItem
    }
}

pub enum Alias { 
    Als,
	Ifunc
}