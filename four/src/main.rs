fn main() {
    let input = &include_str!("./input");
    let part_one = part_one(input);
    println!("Part one: {}", part_one);

    let part_two = part_two(input);
    println!("Part two: {}", part_two);
}

fn parse_pairs(assignments: &str) -> impl Iterator<Item = Vec<std::ops::RangeInclusive<u32>>> + '_ {
    assignments.lines().map(|line| {
        line.split(',')
            .map(|range| {
                let mut parts = range.split('-');
                let start = parts.next().unwrap().parse::<u32>().unwrap();
                let end = parts.next().unwrap().parse::<u32>().unwrap();
                start..=end
            })
            .collect::<Vec<_>>()
    })
}

fn part_one(assignments: &str) -> usize {
    // split by line, then by comma, then parse each pair as a range
    parse_pairs(assignments)
        // filter ranges that are included in another range
        .filter(|ranges| {
            ranges[0].start() >= ranges[1].start() && ranges[0].end() <= ranges[1].end()
                || ranges[0].start() <= ranges[1].start() && ranges[0].end() >= ranges[1].end()
        })
        .count()
}

#[test]
fn test_part_one() {
    let assignments = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part_one(assignments), 2);
}

fn part_two(assignments: &str) -> usize {
    parse_pairs(assignments)
        // filter by ranges that partially overlap
        .filter(|ranges| {
            ranges[0].end() >= ranges[1].start() && ranges[1].end() >= ranges[0].start()
        })
        .count()
}

#[test]
fn test_part_two() {
    let assignments = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part_two(assignments), 4);
}
