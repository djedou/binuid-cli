use super::TypeConst;


pub enum StructConst {
    GrOrLessParath {
        type_list: Vec<TypeConst>
    },
    GrOrLessParathEmpty,
    OnlyParath {
        type_list: Vec<TypeConst>
    },
    OnlyParathEmpty
}