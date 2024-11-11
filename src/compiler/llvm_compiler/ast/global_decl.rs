use super::{ Type, Ident, ExternLinkage, PreemptionSpecifier, Visibility, DLLStorageClass, 
    ThreadLocal, UnnamedAddr, Immutable, GlobalAttr, FuncAttr, AddrSpace
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct GlobalDecl {
    pub ident: Ident,
    pub extern_linkage: ExternLinkage,
    pub preemption_specifier: PreemptionSpecifier,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass,
    pub thread_local: ThreadLocal,
    pub unnamed_addr: UnnamedAddr,
    pub addr_space: AddrSpace,
    pub externally_initialized: bool,
    pub immutable: Immutable,
    pub type_: Type,
    pub global_attrs: Vec<GlobalAttr>,
    pub func_attrs: Vec<FuncAttr>
}



impl BuildFrom for GlobalDecl {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> GlobalDecl {
        let mut global = GlobalDecl {
            ident: Ident::None,
            extern_linkage: ExternLinkage::None,
            preemption_specifier: PreemptionSpecifier::None,
            visibility: Visibility::None,
            dll_storage_class: DLLStorageClass::None,
            thread_local: ThreadLocal::new(),
            unnamed_addr: UnnamedAddr::None,
            addr_space: AddrSpace::new(),
            externally_initialized: false,
            immutable: Immutable::None,
            type_: Type::None,
            global_attrs: vec![],
            func_attrs: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::GlobalIdent => {
                    global.ident = Ident::build_from(&inner_pair);
                },
                Rule::ExternLinkage => {
                    global.extern_linkage = ExternLinkage::build_from(&inner_pair);
                },
                Rule::PreemptionSpecifier => {
                    global.preemption_specifier = PreemptionSpecifier::build_from(&inner_pair);
                },
                Rule::Visibility => {
                    global.visibility = Visibility::build_from(&inner_pair);
                },
                Rule::DLLStorageClass => {
                    global.dll_storage_class = DLLStorageClass::build_from(&inner_pair);
                },
                Rule::ThreadLocal => {
                    global.thread_local = ThreadLocal::build_from(&inner_pair);
                },
                Rule::UnnamedAddr => {
                    global.unnamed_addr = UnnamedAddr::build_from(&inner_pair);
                },
                Rule::AddrSpace => {
                    global.addr_space = AddrSpace::build_from(&inner_pair);
                },
                Rule::ExternallyInitialized => {
                    global.externally_initialized = true;
                },
                Rule::Immutable => {
                    global.immutable = Immutable::build_from(&inner_pair);
                },
                Rule::Type => {
                    global.type_ = Type::build_from(&inner_pair);
                },
                Rule::GlobalAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::GlobalAttr => {
                                global.global_attrs.push(GlobalAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::FuncAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::FuncAttr => {
                                global.func_attrs.push(FuncAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        global
    }
}