use crate::shared;

pub(crate) fn part_two(assignments: &str) -> usize {
    shared::parse_pairs(assignments)
        // filter by ranges that partially overlap
        .filter(|ranges| {
            ranges[0].end() >= ranges[1].start() && ranges[1].end() >= ranges[0].start()
        })
        .count()
}

#[test]
pub(crate) fn test_part_two() {
    let assignments = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part_two(assignments), 4);
}
