
#[derive(Debug)]
pub enum Ident {
    Name {
        value: String
    },
    Id {
        value: u32
    }
}