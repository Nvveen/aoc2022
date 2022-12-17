pub fn part_two(mut groups: Vec<i32>) -> i32 {
    // Sort by largest number
    groups.sort_by(|a, b| b.cmp(a));

    // Get the sum of the first 3 elements
    groups.iter().take(3).sum()
}

// test part_two
#[test]
fn test_part_two() {
    // Test part_two with the groups
    let sum = part_two(vec![15924, 11346, 15924, 11346]);
    // Assert that the sum is correct
    assert_eq!(sum, 43194);
}
