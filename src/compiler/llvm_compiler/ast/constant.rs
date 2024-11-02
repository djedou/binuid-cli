use super::{FloatLit, StructConst, ArrayConst, VectorConst, Ident, BlockAddressConst, ConstantExpr};

#[derive(Debug)]
pub enum Constant { 
    Bool {
        value: bool
    },
	Int {
        value: i64
    },
	Float {
        value: FloatLit
    },
	Null,
	None,
	Struct {
        value: StructConst
    },
	Array {
        value: ArrayConst
    },
	CharArray {
        value: String
    },
	Vector {
        value: VectorConst
    },
	ZeroInitializer,
	GlobalIdent {
        ident: Ident
    },
	Undef,
	BlockAddress {
        address: BlockAddressConst
    },
	Expr {
        expr: Box<ConstantExpr>
    }
}
