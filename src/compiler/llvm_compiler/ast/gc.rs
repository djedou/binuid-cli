
#[derive(Debug)]
pub enum GC {
    None,
    String {
        value: String
    }
}