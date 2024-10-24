
pub struct FunctionHeader {
    PreemptionSpecifier: Option<PreemptionSpecifier>,
    Visibility: Option<Visibility>,
    DLLStorageClass: Option<DLLStorageClass>,
    CallingConv: Option<CallingConv>,
    ReturnAttrs: Option<ReturnAttrs>,
    type_: Type,
    GlobalIdent: Ident,
    Params: Vec<Params>,
    UnnamedAddr: Option<UnnamedAddr>,
    FuncAttrs: Vec<FuncAttr>,
    Section: Option<Section>,
    Comdat: Option<Comdat>,
    OptGC: Option<OptGC>,
    OptPrefix: Option<OptPrefix>,
    OptPrologue: Option<OptPrologue>,
    OptPersonality: Option<OptPersonality>
}