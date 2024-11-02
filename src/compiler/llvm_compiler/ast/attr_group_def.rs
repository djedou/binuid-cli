use super::{FuncAttr, Ident};

#[derive(Debug)]
pub struct AttrGroupDef {
    pub attr_group_id: Ident,
    pub func_attrs: Vec<FuncAttr>
}