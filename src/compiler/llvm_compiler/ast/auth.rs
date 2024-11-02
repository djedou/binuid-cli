
#[derive(Debug)]
pub enum Auth {
    Readwrite,
    Writeonly,
    Readonly,
    Read,
    Write,
}