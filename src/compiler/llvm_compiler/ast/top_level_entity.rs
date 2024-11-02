use super::{ Type, TypeDef, GlobalDecl,
    GlobalDef, IndirectSymbolDef, AttrGroupDef, MetadataNode, MetadataName, 
    MetadataDef, UseListOrderBB, FunctionDef, FunctionDecl, UseListOrder
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
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


impl BuildFrom for TopLevelEntity {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TopLevelEntity {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::SourceFilename => {
                        TopLevelEntity::None
                    },
                    _ => TopLevelEntity::None
                }
            }
            None => TopLevelEntity::None
        }
    }
}