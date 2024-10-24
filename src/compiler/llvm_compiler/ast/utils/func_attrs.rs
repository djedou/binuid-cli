

pub enum FuncAttr {
    AttrGroupId {
        id: u32
    },
    Align {
        int: u32
    },
    Alignstack {
        int: u32
    },
    String {
        string: String
    },
    Alignment,
    AllocSize,
    StackAlignment,
    StringLit,
    Alwaysinline,
    Argmemonly,
    Builtin,
    Cold,
    Convergent,
    InaccessiblememOrArgmemonly,
    Inaccessiblememonly,
    Inlinehint,
    Jumptable,
    Minsize,
    Naked,
    Nobuiltin,
    Noduplicate,
    Noimplicitfloat,
    Noinline,
    Nonlazybind,
    Norecurse,
    Noredzone,
    Noreturn,
    Nounwind,
    Optnone,
    Optsize,
    Readnone,
    Readonly,
    ReturnsTwice,
    Safestack,
    SanitizeAddress,
    SanitizeHwaddress,
    SanitizeMemory,
    SanitizeThread,
    Speculatable,
    Ssp,
    Sspreq,
    Sspstrong,
    Strictfp,
    Uwtable,
    Uriteonly,
    Memory,
    Mustprogress,
    Nofree,
    Nosync,
    Willreturn,
    Mocallback,
    Allockind
}