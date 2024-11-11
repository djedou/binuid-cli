use super::Sign;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum FloatLit {
    None,
    Frac {
        value: FracLit
    },
	Sci {
        value: SciLit
    },
	FloatHex {
        value: FloatHexLit
    }
}

#[derive(Debug)]
pub struct FracLit {
    pub sign: Sign,
    pub decimals: u32,
    pub after_dot: u32
}

#[derive(Debug)]
pub struct SciLit {
    pub frac: FracLit,
    pub exp_sign: Sign,
    pub exp_decimals: u32
}

#[derive(Debug)]
pub struct FloatHexLit {
    pub type_: FloatHexLitType,
    pub value: Vec<u8>
}


#[derive(Debug)]
pub enum FloatHexLitType {
    None,
    HexFP,
    HexFP80,
    HexFP128,
    HexPPC12,
    HexHal
}


impl BuildFrom for FloatLit {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FloatLit {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::FracLit => FloatLit::Frac {
                        value: FracLit::build_from(&inner_pair)
                    },
                    Rule::SciLit => FloatLit::Sci {
                        value: SciLit::build_from(&inner_pair)
                    },
                    Rule::FloatHexLit => FloatLit::FloatHex {
                        value: FloatHexLit::build_from(&inner_pair)
                    },
                    _ => FloatLit::None
                }
            },
            None => FloatLit::None
        }
    }
}

impl FracLit {
    fn new() -> FracLit {
        FracLit {
            sign: Sign::None,
            decimals: 0,
            after_dot: 0
        }
    }
}

impl BuildFrom for FracLit {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FracLit {
        let mut frac = FracLit::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Sign => {
                    frac.sign = Sign::build_from(&inner_pair);
                },
                Rule::Decimals => {
                    frac.decimals = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                Rule::DecimalPart => {
                    frac.after_dot = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                _ => {}
            }
        }

        frac
    }
}

impl BuildFrom for SciLit {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> SciLit {
        let mut sci = SciLit {
            frac: FracLit::new(),
            exp_sign: Sign::None,
            exp_decimals: 0
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::FracLit => {
                    sci.frac = FracLit::build_from(&inner_pair);
                },
                Rule::Sign => {
                    sci.exp_sign = Sign::build_from(&inner_pair);
                },
                Rule::Decimals => {
                    sci.exp_decimals = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                _ => {}
            }
        }

        sci
    }
}

impl BuildFrom for FloatHexLit {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FloatHexLit {
        println!("FloatHexLit is not yet implemented!");

        FloatHexLit {
            type_: FloatHexLitType::None,
            value: Vec::with_capacity(0)
        }
        /*for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::GlobalIdent => {
                    global.item = Ident::build_from(&inner_pair);
                },
                _ => {}
            }
        }*/
    }
}