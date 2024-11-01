use super::{MDTuple, MetadataId, MDString};

pub enum MDNode {
    Tuple {
        value: MDTuple
    },
	Id {
        id: MetadataId
    },
    String {
        data: MDString
    }
}