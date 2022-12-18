use std::fs;

mod part_one;
mod part_two;

static FILENAME: &str = "./two/src/input";

fn main() {
    // read the file
    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the file");
    // split the input into lines
    let lines = contents.split('\n').collect::<Vec<&str>>();

    let score_one = part_one::part_one(&lines);
    println!("Part one total score: {}", score_one);

    let score_two = part_two::part_two(&lines);
    println!("Part two total score: {}", score_two);
}
