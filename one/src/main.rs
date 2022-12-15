use std::fs;

static FILENAME: &str = "./one/src/input";

fn main() {
    println!("Reading file {FILENAME}");

    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the input");

    println!("With text:\n{contents}");
}
