use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
  name: String,
  age: u8,
  skills: Vec<String>,
}

fn main() -> Result<()> {
  let user = User {
    name: "Bob".to_string(),
    age: 25,
    skills: vec!["Rust".to_string(), "Python".to_string()],
  };

  let json = serde_json::to_string(&user)?;
  println!("{:?}", json);

  let user1: User = serde_json::from_str(&json)?;
  print!("{:?}", user1);

  assert_eq!(user, user1);

  Ok(())
}