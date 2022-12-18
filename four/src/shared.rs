pub(crate) fn parse_pairs(
    assignments: &str,
) -> impl Iterator<Item = Vec<std::ops::RangeInclusive<u32>>> + '_ {
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
