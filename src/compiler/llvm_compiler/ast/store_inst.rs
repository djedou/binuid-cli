use super::{TypeValue, AtomicOrdering, Alignment, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum StoreInst {
    None,
    Atomic1 {
        value: AtomicStore1
    },
	Atomic2 {
        value: AtomicStore2
    },
	Volatile1 {
        value: VolatileStore1
    },
	Volatile2 {
        value: VolatileStore2
    }
}


#[derive(Debug)]
pub struct AtomicStore1 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub align: Alignment,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct AtomicStore2 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub order: AtomicOrdering,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct VolatileStore1 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub align: Alignment,
    pub metadata_attachments: Vec<MetadataAttachment>
}

#[derive(Debug)]
pub struct VolatileStore2 {
    pub volatile: bool,
    pub lhs: TypeValue,
    pub rhs: TypeValue,
    pub metadata_attachments: Vec<MetadataAttachment>
}

impl BuildFrom for AtomicStore1 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AtomicStore1 {
        let mut target = AtomicStore1 {
            volatile: false,
            lhs: TypeValue::new(),
            rhs: TypeValue::new(),
            order: AtomicOrdering::None,
            align: Alignment {
                int: 0
            },
            metadata_attachments: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    target.volatile = true;
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.lhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachments => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::MetadataAttachment => {
                                target.metadata_attachments.push(MetadataAttachment::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::AtomicOrdering => {
                    target.order = AtomicOrdering::build_from(&inner_pair);
                },
                Rule::Alignment => {
                    target.align = Alignment::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        target
    }
}


impl BuildFrom for AtomicStore2 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AtomicStore2 {
        let mut target = AtomicStore2 {
            volatile: false,
            lhs: TypeValue::new(),
            rhs: TypeValue::new(),
            order: AtomicOrdering::None,
            metadata_attachments: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    target.volatile = true;
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.lhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachments => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::MetadataAttachment => {
                                target.metadata_attachments.push(MetadataAttachment::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::AtomicOrdering => {
                    target.order = AtomicOrdering::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        target
    }
}


impl BuildFrom for VolatileStore1 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VolatileStore1 {
        let mut target = VolatileStore1 {
            volatile: false,
            lhs: TypeValue::new(),
            rhs: TypeValue::new(),
            align: Alignment {
                int: 0
            },
            metadata_attachments: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    target.volatile = true;
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.lhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachments => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::MetadataAttachment => {
                                target.metadata_attachments.push(MetadataAttachment::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::Alignment => {
                    target.align = Alignment::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        target
    }
}



impl BuildFrom for VolatileStore2 {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VolatileStore2 {
        let mut target = VolatileStore2 {
            volatile: false,
            lhs: TypeValue::new(),
            rhs: TypeValue::new(),
            metadata_attachments: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Volatile => {
                    target.volatile = true;
                },
                Rule::LhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.lhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::RhsTypeValue => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                target.rhs = TypeValue::build_from(&p);
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachments => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::MetadataAttachment => {
                                target.metadata_attachments.push(MetadataAttachment::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        target
    }
}


impl BuildFrom for StoreInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> StoreInst {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::AtomicStore1 => StoreInst::Atomic1 {
                        value: AtomicStore1::build_from(&inner_pair)
                    },
                    Rule::AtomicStore2 => StoreInst::Atomic2 {
                        value: AtomicStore2::build_from(&inner_pair)
                    },
                    Rule::VolatileStore1 => StoreInst::Volatile1 {
                        value: VolatileStore1::build_from(&inner_pair)
                    },
                    Rule::VolatileStore2 => StoreInst::Volatile2 {
                        value: VolatileStore2::build_from(&inner_pair)
                    },
                    _ => StoreInst::None
                }
            },
            None => StoreInst::None
        }
    }
}