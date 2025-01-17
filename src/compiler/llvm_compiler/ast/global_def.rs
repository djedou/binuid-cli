use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::{AddrSpace, Visibility, UnnamedAddr, Linkage, Immutable, PreemptionSpecifier, DLLStorageClass},
    composes::{ThreadLocal, Ident, GlobalAttr, FuncAttr, GlobalDef},
    types::Type,
    values::Constant
};



impl BuildFrom for GlobalDef {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> GlobalDef {
        let mut global = GlobalDef {
            ident: Ident::None,
            linkage: Linkage::None,
            preemption_specifier: PreemptionSpecifier::None,
            visibility: Visibility::None,
            dll_storage_class: DLLStorageClass::None,
            thread_local: ThreadLocal::new(),
            unnamed_addr: UnnamedAddr::None,
            addr_space: AddrSpace::default(),
            externally_initialized: false,
            immutable: Immutable::None,
            type_: Type::None,
            constant: Constant::None,
            global_attrs: vec![],
            func_attrs: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::GlobalIdent => {
                    global.ident = Ident::build_from(&inner_pair);
                },
                Rule::Linkage => {
                    global.linkage = Linkage::build_from(&inner_pair);
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
                Rule::Constant => {
                    global.constant = Constant::build_from(&inner_pair);
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