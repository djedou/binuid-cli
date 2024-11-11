use super::{ Type, TypeDef, GlobalDecl,
    GlobalDef, IndirectSymbolDef, AttrGroupDef, MetadataNode, MetadataName, 
    MetadataDef, UseListOrderBB, FunctionDef, FunctionDecl, UseListOrder, TargetDefinition
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum TopLevelEntity {
    None,
    SourceFilename  {
        value: String
    },
	TargetDefinition {
        target: TargetDefinition
    },
	ModuleAsm {
        value: String
    },
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
        def: IndirectSymbolDef
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
                        match inner_pair.clone().into_inner().next() {
                            Some(p) => {
                                match p.as_rule() {
                                    Rule::StringLit => {
                                        TopLevelEntity::SourceFilename {
                                            value: String::build_from(&p)
                                        }
                                    },
                                    _ => TopLevelEntity::None
                                }
                            },
                            None => TopLevelEntity::None
                        }
                    },
                    Rule::TargetDefinition => {
                        TopLevelEntity::TargetDefinition {
                            target: TargetDefinition::build_from(&inner_pair)
                        }
                    },
                    Rule::ModuleAsm => {
                        match inner_pair.clone().into_inner().next() {
                            Some(p) => {
                                match p.as_rule() {
                                    Rule::StringLit => {
                                        TopLevelEntity::ModuleAsm {
                                            value: String::build_from(&p)
                                        }
                                    },
                                    _ => TopLevelEntity::None
                                }
                            },
                            None => TopLevelEntity::None
                        }
                    },
                    Rule::TypeDef => {
                        TopLevelEntity::TypeDef {
                            def: TypeDef::build_from(&inner_pair)
                        }
                    },
                    Rule::GlobalDecl => {
                        TopLevelEntity::GlobalDecl {
                            decl: GlobalDecl::build_from(&inner_pair)
                        }
                    },
                    Rule::GlobalDef => {
                        TopLevelEntity::GlobalDef {
                            def: GlobalDef::build_from(&inner_pair)
                        }
                    },
                    Rule::IndirectSymbolDef => {
                        TopLevelEntity::IndirectSymbolDef {
                            def: IndirectSymbolDef::build_from(&inner_pair)
                        }
                    },
                    Rule::FunctionDecl => {
                        TopLevelEntity::FunctionDecl {
                            decl: FunctionDecl::build_from(&inner_pair)
                        }
                    },
                    Rule::FunctionDef => {
                        TopLevelEntity::FunctionDef {
                            def: FunctionDef::build_from(&inner_pair)
                        }
                    },
                    Rule::AttrGroupDef => {
                        TopLevelEntity::AttrGroupDef {
                            def: AttrGroupDef::build_from(&inner_pair)
                        }
                    },
                    Rule::MetadataDef => {
                        TopLevelEntity::MetadataDef {
                            def: MetadataDef::build_from(&inner_pair)
                        }
                    },
                    Rule::UseListOrder => {
                        TopLevelEntity::UseListOrder {
                            order: UseListOrder::build_from(&inner_pair)
                        }
                    },
                    Rule::UseListOrderBB => {
                        TopLevelEntity::UseListOrderBB {
                            order: UseListOrderBB::build_from(&inner_pair)
                        }
                    },
                    Rule::NamedMetadataDef => {
                        let mut name = MetadataName::new();
                        let mut nodes = vec![];
                        
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::MetadataName => {
                                    name = MetadataName::build_from(&p);
                                }
                                Rule::MetadataNode => {
                                    nodes.push(MetadataNode::build_from(&p));
                                },
                                _ => {}
                            }
                        }
                        TopLevelEntity::NamedMetadataDef {
                            metadata_name: name,
                            metadata_nodes: nodes
                        }
                    },
                    /*
                        | ComdatDef
                    */
                    _ => TopLevelEntity::None
                }
            }
            None => TopLevelEntity::None
        }
    }
}