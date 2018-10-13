extern crate config;
extern crate ripsaw;

use std::io;
use std::collections::HashMap;

use ripsaw::Lumber;
use ripsaw::CutList;
use ripsaw::Settings;

fn main() {
    let mut settings = config::Config::default();
    let mut settings_hashmap: HashMap<String, String> = HashMap::new();
    settings
        .merge(config::File::with_name("ripsaw-config"))
        .unwrap()
        .merge(config::Environment::with_prefix("RIPSAW"))
        .unwrap();

    // Deserialize to a hash map.
    settings_hashmap = settings.try_into::<HashMap<String, String>>().unwrap();
    let settings = Settings::new_from_hashmap(settings_hashmap);

    let mut input;
    let mut cut_list = CutList::new_with_settings(settings);

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
