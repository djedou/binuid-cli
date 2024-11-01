use super::{FuncAttr, Ident};


pub struct AttrGroupDef {
    pub attr_group_id: Ident,
    pub func_attrs: Vec<FuncAttr>
}