use super::{BinaryExpr, BitwiseExpr, VectorExpr, AggregateExpr, MemoryExpr, ConversionExpr, OtherExp};


#[derive(Debug)]
pub enum ConstantExpr {
    Binary {
        expr: BinaryExpr
    },
	Bitwise {
        expr: BitwiseExpr
    },
	Vector {
        expr: VectorExpr
    },
	Aggregate {
        expr: AggregateExpr
    },
	Memory {
        expr: MemoryExpr
    },
	Conversion {
        expr: ConversionExpr
    },
	Other {
        expr: OtherExp
    }
}