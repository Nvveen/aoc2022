pub(crate) fn part_two(contents: &str) -> u32 {
    // split by newlines and collect in groups of 3
    contents
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            // find character that appears in all 3 strings
            let common: String = group[0]
                .chars()
                .filter(|c| group[1].contains(*c) && group[2].contains(*c))
                .collect::<std::collections::HashSet<char>>()
                .iter()
                .collect();
            // convert to priority and sum
            common
                .chars()
                .map(|c| {
                    if c.is_lowercase() {
                        c as u32 - 96
                    } else {
                        c as u32 - 38
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

// test part_two
#[test]
pub(crate) fn test_part_two() {
    let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    assert_eq!(part_two(contents), 70);
}
