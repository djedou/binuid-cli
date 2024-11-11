use super::{Type, TypeValue, Alignment, MetadataAttachment, AddrSpace};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum AllocaInst {
    None,
    AllocaInst1 {
        inst: AllocaInst1
    },
	AllocaInst2 {
        inst: AllocaInst2
    },
	AllocaInst3 {
        inst: AllocaInst3
    },
	AllocaInst4 {
        inst: AllocaInst4
    },
	AllocaInst5 {
        inst: AllocaInst5
    },
	AllocaInst6 {
        inst: AllocaInst6
    },
	AllocaInst7 {
        inst: AllocaInst7
    },
	AllocaInst8 {
        inst: AllocaInst8
    }
}

#[derive(Debug)]
pub struct AllocaInst1 {
    pub in_alloca: bool,
    pub swift_error: bool,
    pub type_: Type,
    pub rhs: TypeValue,
    pub align: Alignment,
    pub addr_space: AddrSpace,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AllocaInst2 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    rhs: TypeValue,
    addr_space: AddrSpace,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AllocaInst3 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    rhs: TypeValue,
    align: Alignment,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AllocaInst4 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    align: Alignment,
    addr_space: AddrSpace,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AllocaInst5 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    rhs: TypeValue,
    metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AllocaInst6 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    align: Alignment,
    metadata_attachments: Vec<MetadataAttachment>
}

impl AllocaInst6 {
    pub fn new() -> AllocaInst6 {
        AllocaInst6 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            align: Alignment::new(),
            metadata_attachments: vec![]
        }
    }
}

impl AllocaInst5 {
    pub fn new() -> AllocaInst5 {
        AllocaInst5 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct AllocaInst7 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    addr_space: AddrSpace,
    metadata_attachments: Vec<MetadataAttachment>
}


impl AllocaInst7 {
    pub fn new() -> AllocaInst7 {
        AllocaInst7 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            addr_space: AddrSpace::new(),
            metadata_attachments: vec![]
        }
    }
}

#[derive(Debug)]
pub struct AllocaInst8 {
    in_alloca: bool,
    swift_error: bool,
    type_: Type,
    metadata_attachments: Vec<MetadataAttachment>
}

impl AllocaInst8 {
    pub fn new() -> AllocaInst8 {
        AllocaInst8 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            metadata_attachments: vec![]
        }
    }
}

impl AllocaInst1 {
    pub fn new() -> AllocaInst1 {
        AllocaInst1 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            align: Alignment::new(),
            addr_space: AddrSpace::new(),
            metadata_attachments: vec![]
        }
    }
}

impl AllocaInst2 {
    pub fn new() -> AllocaInst2 {
        AllocaInst2 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            addr_space: AddrSpace::new(),
            metadata_attachments: vec![]
        }
    }
}

impl AllocaInst4 {
    pub fn new() -> AllocaInst4 {
        AllocaInst4 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            align: Alignment::new(),
            addr_space: AddrSpace::new(),
            metadata_attachments: vec![]
        }
    }
}

impl AllocaInst3 {
    pub fn new() -> AllocaInst3 {
        AllocaInst3 {
            in_alloca: false,
            swift_error: false,
            type_: Type::None,
            rhs: TypeValue::new(),
            align: Alignment::new(),
            metadata_attachments: vec![]
        }
    }
}

impl BuildFrom for AllocaInst1 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst1 {
        let mut item = AllocaInst1::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
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
                Rule::AddrSpace => {
                    item.addr_space = AddrSpace::build_from(&inner_pair);
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



impl BuildFrom for AllocaInst7 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst7 {
        let mut item = AllocaInst7::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::AddrSpace => {
                    item.addr_space = AddrSpace::build_from(&inner_pair);
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


impl BuildFrom for AllocaInst6 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst6 {
        let mut item = AllocaInst6::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
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


impl BuildFrom for AllocaInst5 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst5 {
        let mut item = AllocaInst5::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
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



impl BuildFrom for AllocaInst8 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst8 {
        let mut item = AllocaInst8::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
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




impl BuildFrom for AllocaInst2 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst2 {
        let mut item = AllocaInst2::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
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
                Rule::AddrSpace => {
                    item.addr_space = AddrSpace::build_from(&inner_pair);
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


impl BuildFrom for AllocaInst3 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst3 {
        let mut item = AllocaInst3::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
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


impl BuildFrom for AllocaInst4 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst4 {
        let mut item = AllocaInst4::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::InAlloca => {
                    item.in_alloca = true;
                },
                Rule::SwiftError => {
                    item.swift_error = true;
                },
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::Alignment => {
                    item.align = Alignment::build_from(&inner_pair);
                },
                Rule::AddrSpace => {
                    item.addr_space = AddrSpace::build_from(&inner_pair);
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

impl BuildFrom for AllocaInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AllocaInst {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::AllocaInst1 => AllocaInst::AllocaInst1 {
                        inst: AllocaInst1::build_from(&inner_pair)
                    },
                    Rule::AllocaInst2 => AllocaInst::AllocaInst2 {
                        inst: AllocaInst2::build_from(&inner_pair)
                    },
                    Rule::AllocaInst3 => AllocaInst::AllocaInst3 {
                        inst: AllocaInst3::build_from(&inner_pair)
                    },
                    Rule::AllocaInst4 => AllocaInst::AllocaInst4 {
                        inst: AllocaInst4::build_from(&inner_pair)
                    },
                    Rule::AllocaInst5 => AllocaInst::AllocaInst5 {
                        inst: AllocaInst5::build_from(&inner_pair)
                    },
                    Rule::AllocaInst6 => AllocaInst::AllocaInst6 {
                        inst: AllocaInst6::build_from(&inner_pair)
                    },
                    Rule::AllocaInst7 => AllocaInst::AllocaInst7 {
                        inst: AllocaInst7::build_from(&inner_pair)
                    },
                    Rule::AllocaInst8 => AllocaInst::AllocaInst8 {
                        inst: AllocaInst8::build_from(&inner_pair)
                    },
                    _ => AllocaInst::None
                }
            },
            None => AllocaInst::None
        }
    }
}