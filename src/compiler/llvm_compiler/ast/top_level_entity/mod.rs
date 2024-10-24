use super::{
    types::{Type, TypeDef},
    ExternLinkage,
    indirect_symbol_def::IndirectSymbolDef
};

pub enum TopLevelEntity {
    None,
    TypeDef {
        def: TypeDef
    },
    GlobalDecl {
        ident: Ident,
        extern_linkage: ExternLinkage,
        preemption_specifier: Option<PreemptionSpecifier>,
        visibility: Option<Visibility>,
        dll_storage_class: Option<DLLStorageClass>,
        thread_local: Option<ThreadLocal>,
        unnamed_addr: Option<UnnamedAddr>,
        addr_space: Option<AddrSpace>,
        externally_initialized: bool,
        immutable: Immutable,
        type_: Type,
        global_attrs: Vec<GlobalAttr>,
        func_attrs: Vec<FuncAttr>
    },
    GlobalDef {
        ident: Ident,
        linkage: Option<Linkage>,
        preemption_specifier: Option<PreemptionSpecifier>,
        visibility: Option<Visibility>,
        dll_storage_class: Option<DLLStorageClass>,
        thread_local: Option<ThreadLocal>,
        unnamed_addr: Option<UnnamedAddr>,
        addr_space: Option<AddrSpace>,
        externally_initialized: bool,
        immutable: Immutable,
        type_: Type,
        constant: Constant,
        global_attrs: Vec<GlobalAttr>,
        func_attrs: Vec<FuncAttr>
    },
	IndirectSymbolDef {
        def: Option<IndirectSymbolDef>
    },
	FunctionDecl {
        metadata_attachments: Vec<MetadataAttachment>,
        extern_linkage: Option<ExternLinkage>,
        function_header: FunctionHeader
    },
	FunctionDef {
        linkage: Option<Linkage>,
        function_header: FunctionHeader,
        metadata_attachments: Vec<MetadataAttachment>,
        function_body: FunctionBody
    },
	AttrGroupDef {
        attributes: Attributes,
        attr_group_id: AttrGroupId,
        func_attrs: FuncAttrs
    },
	NamedMetadataDef {
        metadata_name: MetadataName,
        metadata_nodes: Option<MetadataNodes>
    },
    MetadataDef {
        def: MetadataDef
    },
    UseListOrder {
        order: UseListOrder
    },
	UseListOrderBB {
        global_ident: Ident,
        local_ident: Ident,
        indices: Option<Indices>
    }
}