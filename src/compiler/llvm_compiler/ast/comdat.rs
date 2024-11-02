
#[derive(Debug)]
pub enum Comdat {
    Empty,
    Name {
        name: String
    }
}