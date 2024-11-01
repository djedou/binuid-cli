use super::{Sign};

pub enum FloatLit { 
    Frac {
        sign: Sign,
        decimals: u32,
        after_dot: u32
    },
	Sci {
        sign: Sign,
        decimals: u32,
        after_dot: u32,
        exp_sign: Sign,
        exp_decimals: u32
    },
	FloatHex {
        type_: FloatHexLit,
        value: Vec<u8>
    }
}

pub enum FloatHexLit {
    HexFP,
    HexFP80,
    HexFP128,
    HexPPC12,
    HexHal
}
