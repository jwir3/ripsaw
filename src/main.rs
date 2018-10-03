extern crate ripsaw;

use std::io;

use ripsaw::Lumber;

fn main() {
    let mut input = String::new();

    println!("Enter your lumber need: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read lumber input string");

    let lumber = Lumber::create_from_spec(input);
    println!("Lumber needed: {}", lumber);
}
