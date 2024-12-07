use super::{Type, TypeValue, AtomicOrdering, Alignment, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum LoadInst {
    None,
	// Load.
	LoadInst1 {
        inst: LoadInst1
    },
	// Atomic load.
	LoadInst2 {
        inst: LoadInst2
    },
	LoadInst3 {
        inst: LoadInst3
    },
    LoadInst4 {
        inst: LoadInst4
    },
}

#[derive(Debug)]
pub struct LoadInst1 {
    volatile: bool,
    type_: Type,
    rhs: TypeValue,
    order: AtomicOrdering,
    align: Alignment,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct LoadInst2 {
    volatile: bool,
    type_: Type,
    rhs: TypeValue,
    order: AtomicOrdering,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct LoadInst3 {
    volatile: bool,
    type_: Type,
    rhs: TypeValue,
    align: Alignment,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct LoadInst4 {
    volatile: bool,
    type_: Type,
    rhs: TypeValue,
    metadata_attachments: Vec<MetadataAttachment>
}


impl LoadInst1 {
    pub fn new() -> LoadInst1 {
        LoadInst1 {
            volatile: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            order: AtomicOrdering::None,
            align: Alignment::new(),
            metadata_attachments: vec![]
        }
    }
}

impl LoadInst3 {
    pub fn new() -> LoadInst3 {
        LoadInst3 {
            volatile: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            align: Alignment::new(),
            metadata_attachments: vec![]
        }
    }
}

impl LoadInst2 {
    pub fn new() -> LoadInst2 {
        LoadInst2 {
            volatile: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            order: AtomicOrdering::None,
            metadata_attachments: vec![]
        }
    }
}

impl LoadInst4 {
    pub fn new() -> LoadInst4 {
        LoadInst4 {
            volatile: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for LoadInst1 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LoadInst1 {
        let mut item = LoadInst1::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    item.volatile = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Alignment => {
                    item.align = Alignment::build_from(&inner_pair);
                },
                Rule::AtomicOrdering => {
                    item.order = AtomicOrdering::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for LoadInst2 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LoadInst2 {
        let mut item = LoadInst2::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    item.volatile = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::AtomicOrdering => {
                    item.order = AtomicOrdering::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for LoadInst3 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LoadInst3 {
        let mut item = LoadInst3::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    item.volatile = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Alignment => {
                    item.align = Alignment::build_from(&inner_pair);
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for LoadInst4 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LoadInst4 {
        let mut item = LoadInst4::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    item.volatile = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}

impl BuildFrom for LoadInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LoadInst {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::LoadInst1 => LoadInst::LoadInst1 {
                        inst: LoadInst1::build_from(&inner_pair)
                    },
                    Rule::LoadInst2 => LoadInst::LoadInst2 {
                        inst: LoadInst2::build_from(&inner_pair)
                    },
                    Rule::LoadInst3 => LoadInst::LoadInst3 {
                        inst: LoadInst3::build_from(&inner_pair)
                    },
                    Rule::LoadInst4 => LoadInst::LoadInst4 {
                        inst: LoadInst4::build_from(&inner_pair)
                    },
                    _ => LoadInst::None
                }
            },
            None => LoadInst::None
        }
    }
}