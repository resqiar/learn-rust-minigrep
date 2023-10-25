use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let target_file = &args[2];

    println!("{query}");
    println!("{target_file}");
}
