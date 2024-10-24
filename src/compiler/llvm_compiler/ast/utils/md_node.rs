pub enum MDNode {
    MDTuple {
        tuple: MDTuple
    },
	MetadataId {
        id: u32
    }
}

pub enum MDTuple {
    Fields {
        fields: Vec<MDField>
    },
    Empty
}

pub enum MDField {
    Null,
	Metadata {
        metadata: Metadata
    }
}