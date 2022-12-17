pub fn part_one(groups: &[i32]) -> i32 {
    // Get the largest number in groups
    *groups
        .iter()
        .max()
        .expect("Should have been able to get the largest number")
}

// test part_one
#[test]
fn test_part_one() {
    // Test part_one with the groups
    let largest = part_one(&[15924, 11346, 15924, 11346]);
    // Assert that the largest number is correct
    assert_eq!(largest, 15924);
}
