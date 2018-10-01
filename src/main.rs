use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your lumber need: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read lumber input string");
}
