use std::fs;

static FILENAME: &str = "./one/src/input";

fn main() {
    println!("Reading file {FILENAME}");

    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the input");

    // Split the input into lines
    let lines = contents.split('\n');

    // Group lines per empty line
    let groups: Vec<i32> = lines
        .into_iter()
        .fold((Vec::new(), Vec::new()), |(mut groups, mut group), line| {
            if line.is_empty() {
                let sum: i32 = group.iter().sum();
                groups.push(sum);
                (groups, Vec::new())
            } else {
                let number: i32 = line
                    .parse()
                    .expect("Should have been able to parse the number");
                group.push(number);
                (groups, group)
            }
        })
        .0;

    // Get the largest number in groups
    let largest = groups
        .iter()
        .max()
        .expect("Should have been able to get the largest number");

    // print result
    println!("The largest number is {}", largest);
}
