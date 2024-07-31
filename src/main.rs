use anyhow::Context;
use ecosystem::MyError;
use std::fs;

fn main() -> Result<(), anyhow::Error>{
    let filename = "non-existent-file";

    let _fd = fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
