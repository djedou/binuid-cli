use super::{MDTuple, MetadataId, MDString};


#[derive(Debug)]
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