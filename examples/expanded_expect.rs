#![allow(dead_code)]

use public_display::PublicDisplay;
use std::fmt::Display;

#[derive(Debug)]
struct Person {
    pub name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Person))
            .field("name", &self.name)
            .finish()
    }
}

impl PublicDisplay for Person {}

#[derive(Debug)]
struct City(pub String);

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(City)).field(&self.0).finish()
    }
}

impl PublicDisplay for City {}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 20,
    };

    println!("{person:?}");
    println!("{person}");

    let city = City("Tokyo".to_string());

    println!("{city:?}");
    println!("{city}");
}
