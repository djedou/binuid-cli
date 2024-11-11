use crate::compiler::{llvm_compiler::{Rule, Alignment, MemoryArgItem}, BuildFrom};


#[derive(Debug)]
pub enum FuncAttr {
    None,
    AttrGroupId {
        id: u32
    },
    AlignInt {
        int: u32
    },
    Alignstack {
        int: u32
    },
    BinString {
        lhs: String,
        rhs: String
    },
    Alignment {
        align: Alignment
    },
    AllocSize {
        lhs: u32,
        rhs: u32
    },
    StackAlignment {
        value: u32
    },
    String {
        value: String
    },
    Alwaysinline,
    Argmemonly,
    Builtin,
    Cold,
    Convergent,
    InaccessiblememOrArgmemonly,
    Inaccessiblememonly,
    Inlinehint,
    Jumptable,
    Minsize,
    Naked,
    Nobuiltin,
    Noduplicate,
    Noimplicitfloat,
    Noinline,
    Nonlazybind,
    Norecurse,
    Noredzone,
    Noreturn,
    Nounwind,
    Optnone,
    Optsize,
    Readnone,
    Readonly,
    ReturnsTwice,
    Safestack,
    SanitizeAddress,
    SanitizeHwaddress,
    SanitizeMemory,
    SanitizeThread,
    Speculatable,
    Ssp,
    Sspreq,
    Sspstrong,
    Strictfp,
    Uwtable,
    Uriteonly,
    Memory {
        mem: MemoryArgItem
    },
    Mustprogress,
    Nofree,
    Nosync,
    Willreturn,
    Mocallback,
    Allockind {
        value: String
    }
}



impl BuildFrom for FuncAttr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FuncAttr {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::AttrGroupId => {
                        let mut id = 0;
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::Id => {
                                    id = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        }
                        FuncAttr::AttrGroupId {
                            id: id
                        }
                    },
                    Rule::AlignIntLit => {
                        let mut id = 0;
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::IntLit => {
                                    id = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        }

                        FuncAttr::AlignInt {
                            int: id
                        }
                    },
                    Rule::AlignstackIntLit => {
                        let mut id = 0;
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::IntLit => {
                                    id = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        }

                        FuncAttr::Alignstack {
                            int: id
                        }
                    },
                    Rule::BinStringLit => {
                        let mut lhs = String::with_capacity(0);
                        let mut rhs = String::with_capacity(0);
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::LhsStringLit => {
                                    lhs = String::build_from(&p);
                                },
                                Rule::RhsStringLit => {
                                    rhs = String::build_from(&p);
                                },
                                _ => {}
                            }
                        }

                        FuncAttr::BinString {
                            lhs: lhs,
                            rhs: rhs
                        }
                    },
                    Rule::Alignment => FuncAttr::Alignment {
                        align: Alignment::build_from(&inner_pair)
                    },
                    Rule::AllocSize =>  {
                        let mut lhs = 0;
                        let mut rhs = 0;
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::LhsIntLit => {
                                    lhs = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                Rule::RhsIntLit => {
                                    rhs = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        }

                        FuncAttr::AllocSize {
                            lhs: lhs,
                            rhs: rhs
                        }
                    },
                    Rule::StackAlignment => {
                        let mut id = 0;
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::IntLit => {
                                    id = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        }

                        FuncAttr::StackAlignment {
                            value: id
                        }
                    },
                    Rule::StringLit => FuncAttr::String {
                        value: String::build_from(&inner_pair)
                    },
                    Rule::Alwaysinline => FuncAttr::Alwaysinline,
                    Rule::Argmemonly => FuncAttr::Argmemonly,
                    Rule::Builtin => FuncAttr::Builtin,
                    Rule::Cold => FuncAttr::Cold,
                    Rule::Convergent => FuncAttr::Convergent,
                    Rule::InaccessiblememOrArgmemonly => FuncAttr::InaccessiblememOrArgmemonly,
                    Rule::Inaccessiblememonly => FuncAttr::Inaccessiblememonly,
                    Rule::Inlinehint => FuncAttr::Inlinehint,
                    Rule::Jumptable => FuncAttr::Jumptable,
                    Rule::Minsize => FuncAttr::Minsize,
                    Rule::Naked => FuncAttr::Naked,
                    Rule::Nobuiltin => FuncAttr::Nobuiltin,
                    Rule::Noduplicate => FuncAttr::Noduplicate,
                    Rule::Noimplicitfloat => FuncAttr::Noimplicitfloat,
                    Rule::Noinline => FuncAttr::Noinline,
                    Rule::Nonlazybind => FuncAttr::Nonlazybind,
                    Rule::Norecurse => FuncAttr::Norecurse,
                    Rule::Noredzone => FuncAttr::Noredzone,
                    Rule::Noreturn => FuncAttr::Noreturn,
                    Rule::Nounwind => FuncAttr::Nounwind,
                    Rule::Optnone => FuncAttr::Optnone,
                    Rule::Optsize => FuncAttr::Optsize,
                    Rule::Readnone => FuncAttr::Readnone,
                    Rule::Readonly => FuncAttr::Readonly,
                    Rule::ReturnsTwice => FuncAttr::ReturnsTwice,
                    Rule::Safestack => FuncAttr::Safestack,
                    Rule::SanitizeAddress => FuncAttr::SanitizeAddress,
                    Rule::SanitizeHwaddress => FuncAttr::SanitizeHwaddress,
                    Rule::SanitizeMemory => FuncAttr::SanitizeMemory,
                    Rule::SanitizeThread => FuncAttr::SanitizeThread,
                    Rule::Speculatable => FuncAttr::Speculatable,
                    Rule::Ssp => FuncAttr::Ssp,
                    Rule::Sspreq => FuncAttr::Sspreq,
                    Rule::Sspstrong => FuncAttr::Sspstrong,
                    Rule::Strictfp => FuncAttr::Strictfp,
                    Rule::Uwtable => FuncAttr::Uwtable,
                    Rule::Uriteonly => FuncAttr::Uriteonly,
                    Rule::Mustprogress => FuncAttr::Mustprogress,
                    Rule::Nofree => FuncAttr::Nofree,
                    Rule::Nosync => FuncAttr::Nosync,
                    Rule::Willreturn => FuncAttr::Willreturn,
                    Rule::Mocallback => FuncAttr::Mocallback,
                    Rule::Memory => FuncAttr::Memory {
                        mem: MemoryArgItem::build_from(&inner_pair)
                    },
                    Rule::Allockind => {
                        let mut data = String::with_capacity(0);
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::StringLit => {
                                    data = String::build_from(&p);
                                },
                                _ => {}
                            }
                        }

                        FuncAttr::Allockind {
                            value: data
                        }
                    },
                    _ => FuncAttr::None
                }
            },
            None => FuncAttr::None
        }
    }
}