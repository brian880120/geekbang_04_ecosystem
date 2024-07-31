use anyhow::Context;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("BigError: {0:?}")]
    BigError(Box<BigError>),
    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}

fn main() -> Result<(), anyhow::Error>{
    let filename = "non-existent-file";

    let _fd = fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
