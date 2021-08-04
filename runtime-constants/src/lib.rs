pub const MEMSIZE: usize = 1048576;
pub const STDOUT_SIZE: usize = 512;

#[derive(Clone, Copy, Debug)]
pub enum YamType {
    String,
    Uint8,
    Uint16,
    Null,
}
