use std::fs;

static FILENAME: &str = "./one/src/input";

fn main() {
    let contents = read_file(FILENAME);
    let groups = read_groups(contents.as_str());
    part_one(&groups);
    part_two(groups);
}

fn part_one(groups: &[i32]) {
    // Get the largest number in groups
    let largest = groups
        .iter()
        .max()
        .expect("Should have been able to get the largest number");
    // print result
    println!("The largest number is {}", largest);
}

fn part_two(mut groups: Vec<i32>) {
    // Sort by largest number
    groups.sort_by(|a, b| b.cmp(a));

    // Get the sum of the first 3 elements
    let sum: i32 = groups.iter().take(3).sum();

    // print result
    println!("The sum of the top three is {}", sum);
}

// test the read_groups function
#[test]
fn test_read_groups() {
    // string of 10 random 4 digit numbers per line
    let contents = "1234
5678
9012

3456
7890

1234
5678
9012

3456
7890
";
    // Test read_groups with the contents
    let groups = read_groups(contents);
    // Assert that the groups are correct
    assert_eq!(groups, vec![15924, 11346, 15924, 11346]);
}

fn read_groups(contents: &str) -> Vec<i32> {
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
    groups
}

fn read_file(filename: &str) -> String {
    println!("Reading file {filename}");
    let contents = fs::read_to_string(filename).expect("Should have been able to read the input");
    contents
}
