use derive_builder::Builder;
use anyhow::Result;

#[allow(unused)]
#[derive(Builder, Debug)]
struct User {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into, strip_option), default)]
    email: Option<String>,

    #[builder(default = "42")]
    age: u32,

    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User::build()
       .name("Alice")
       .skill("programming")
       .skill("debugging")
       .age(23)
       .email("value@example.com")
       .build()?;
    println!("{:#?}", user);
    Ok(())
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}
