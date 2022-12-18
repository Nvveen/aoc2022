fn main() {
    let part_one = part_one(include_str!("./input"));
    println!("Part one: {}", part_one);
}

fn part_one(assignments: &str) -> usize {
    // split by line, then by comma, then parse each pair as a range
    assignments
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let mut parts = range.split('-');
                    let start = parts.next().unwrap().parse::<u32>().unwrap();
                    let end = parts.next().unwrap().parse::<u32>().unwrap();
                    start..=end
                })
                .collect::<Vec<_>>()
        })
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
