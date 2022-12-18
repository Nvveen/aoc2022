pub(crate) fn part_one(contents: &str) -> u32 {
    // split by newlines
    contents
        .split('\n')
        .map(|line| {
            // find common characters in both halves of the string
            let (first, second) = line.split_at(line.len() / 2);
            let common: String = first
                .chars()
                .filter(|c| second.contains(*c))
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

// test part_one
#[test]
pub(crate) fn test_part_one() {
    let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part_one(contents), 157);
}
