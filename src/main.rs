use std::io;

//1. To create an object in Rust, you define a custom struct and instantiate it by providing values
// for its fields. Unlike Traditional OOP languages, Rust doesn;t use classes; instead, you define
// data in structs and attach behaviors using implementation blocks
pub struct Room {
    name: &'static str,
    n: i32,
    e: i32,
    s: i32,
    w: i32,
}

//2. To create an instance of the struct, declare a variable and define the values for each field.
static HUB: Room = Room {
    name: "Houston",
    n: 1,
    e: 2,
    s: 3,
    w: 4,
};

fn main() {
    println!("Please enter your name:");

    let mut name: String = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {name}");
}
