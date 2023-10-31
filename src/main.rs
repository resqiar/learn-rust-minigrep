use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let target_file = &args[2];

    let file_content = fs::read_to_string(target_file)
        .expect("Unable to read the file. Are you sure the path is correct?");

    println!("{file_content}");
}
