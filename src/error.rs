
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Node Not Found")]
    NotFound,
}