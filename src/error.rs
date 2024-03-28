use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexError {
    /// (expected x, expected y, found x, found y)
    #[error("invalid index (expected x<{0}, y<{1} but got x={2}, y={3}) ")]
    USizeMatrix(usize, usize, usize, usize)
}