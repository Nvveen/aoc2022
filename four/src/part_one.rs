use crate::shared;

pub(crate) fn part_one(assignments: &str) -> usize {
    // split by line, then by comma, then parse each pair as a range
    shared::parse_pairs(assignments)
        // filter ranges that are included in another range
        .filter(|ranges| {
            ranges[0].start() >= ranges[1].start() && ranges[0].end() <= ranges[1].end()
                || ranges[0].start() <= ranges[1].start() && ranges[0].end() >= ranges[1].end()
        })
        .count()
}

#[test]
pub(crate) fn test_part_one() {
    let assignments = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part_one(assignments), 2);
}
