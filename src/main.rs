extern crate ripsaw;

use std::io;

use ripsaw::Lumber;
use ripsaw::CutList;

fn main() {
    let mut input;
    let mut cut_list = CutList::new(None);

    loop {
        input = String::new();
        println!("Enter your lumber need (q to quit): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read lumber input string");

        if input.contains("q") {
            break;
        }

        let lumber = Lumber::create_from_spec(&input);
        cut_list.add(lumber);
    }

    println!("{}", cut_list);
}
