#![allow(dead_code)]

use public_display::PublicDisplay;

#[derive(Debug, PublicDisplay)]
#[public_display(root_visibility)]
struct Person {
    pub name: String,
    #[public_display(skip)]
    pub country: String,
    age: u32,
}

#[derive(Debug, PublicDisplay)]
struct City(pub String);

fn main() {
    let person = Person {
        name: "John".to_string(),
        country: "Japan".to_string(),
        age: 20,
    };

    println!("{person:?}");
    println!("{person}");

    let city = City("Tokyo".to_string());

    println!("{city:?}");
    println!("{city}");
}
