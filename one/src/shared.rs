pub fn read_groups(contents: &str) -> Vec<i32> {
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
