use super::{ Type, TypeDef, GlobalDecl,
    GlobalDef, IndirectSymbolDef, AttrGroupDef, MetadataNode, MetadataName, 
    MetadataDef, UseListOrderBB, FunctionDef, FunctionDecl, UseListOrder
};

pub enum TopLevelEntity {
    None,
    TypeDef {
        def: TypeDef
    },
    GlobalDecl {
        decl: GlobalDecl
    },
    GlobalDef {
        def: GlobalDef,
    },
	IndirectSymbolDef {
        def: Option<IndirectSymbolDef>
    },
	FunctionDecl {
        decl: FunctionDecl
    },
	FunctionDef {
        def: FunctionDef
    },
	AttrGroupDef {
        def: AttrGroupDef
    },
	NamedMetadataDef {
        metadata_name: MetadataName,
        metadata_nodes: Vec<MetadataNode>
    },
    MetadataDef {
        def: MetadataDef
    },
    UseListOrder {
        order: UseListOrder
    },
	UseListOrderBB {
        order: UseListOrderBB
    }
}