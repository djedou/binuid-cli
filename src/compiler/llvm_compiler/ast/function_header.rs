use super::{
    PreemptionSpecifier, Visibility, DLLStorageClass, CallingConv, ReturnAttr, Type, Ident, Param, UnnamedAddr,
    FuncAttr, TypeConst, Section, Comdat
};


#[derive(Debug)]
pub struct FunctionHeader {
    preemption_specifier: Option<PreemptionSpecifier>,
    visibility: Option<Visibility>,
    dll_storage_class: Option<DLLStorageClass>,
    calling_conv: Option<CallingConv>,
    return_attrs: Vec<ReturnAttr>,
    type_: Type,
    global_ident: Ident,
    params: Vec<Param>,
    unnamed_addr: Option<UnnamedAddr>,
    func_attrs: Vec<FuncAttr>,
    section: Option<Section>,
    comdat: Option<Comdat>,
    g: Option<String>,
    prefix: Option<TypeConst>,
    prologue: Option<TypeConst>,
    personality: Option<TypeConst>
}