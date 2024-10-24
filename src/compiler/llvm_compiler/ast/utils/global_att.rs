

pub enum GlobalAttr {
    Section {
        section: String
    },
	Comdat {
        comdat: Comdat
    },
	Align {
        align: u32
    },
	MetadataAttachment {
        attachment: MetadataAttachment
    }
}

pub struct MetadataAttachment {
    name: String,
    node: Option<MDNode>
}

pub enum Comdat {
    Empty,
    Name {
        name: String
    }
}