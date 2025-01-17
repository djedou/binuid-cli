use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    ops::AggregateOp,
    composes::{Index, MetadataAttachment},
    simples::{AddrSpace, Alignment},
    types::{TypeValue, Type}
};
use binuid_shared_wasm::ast_bits::instructions::{
    AllocaInst, AllocaInst1, AllocaInst2, AllocaInst3, 
    AllocaInst4, AllocaInst5, AllocaInst6, AllocaInst7, AllocaInst8
};


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