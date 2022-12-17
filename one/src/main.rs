use part_one::part_one;
use part_two::part_two;
use shared::read_groups;
use std::fs;

mod part_one;
mod part_two;
mod shared;

static FILENAME: &str = "./one/src/input";

fn main() {
    println!("Reading file {FILENAME}");
    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the input");
    let groups = read_groups(contents.as_str());

    let largest = part_one(&groups);
    println!("The largest number is {}", largest);

    let sum = part_two(groups);
    println!("The sum of the top three is {}", sum);
}
