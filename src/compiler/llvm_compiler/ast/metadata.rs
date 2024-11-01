use super::{TypeValue, MDString, MDTuple, MetadataId};


pub enum Metadata {
    TypeValue {
        type_value: TypeValue
    },
    MDString {
        data: MDString
    },
    MDTuple {
        tuple: MDTuple
    },
    MetadataId {
        id: MetadataId
    }
}