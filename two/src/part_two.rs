pub(crate) fn part_two(lines: &[&str]) -> i32 {
    let win = 6;
    let draw = 3;
    let loss = 0;

    let mut sum = 0;
    for line in lines {
        // split line into two teams
        let teams = line.split(' ').collect::<Vec<&str>>();
        let (team_one, team_two) = (teams[0], teams[1]);
        let score = match (team_one, team_two) {
            ("A", "X") => loss + 3,
            ("A", "Y") => draw + 1,
            ("A", "Z") => win + 2,
            ("B", "X") => loss + 1,
            ("B", "Y") => draw + 2,
            ("B", "Z") => win + 3,
            ("C", "X") => loss + 2,
            ("C", "Y") => draw + 3,
            ("C", "Z") => win + 1,
            _ => panic!("Invalid input"),
        };
        sum += score;
    }
    sum
}

// test part_two
#[test]
pub(crate) fn test_part_two() {
    let lines = "A Y\nB X\nC Z".split('\n').collect::<Vec<&str>>();
    assert_eq!(part_two(&lines), 12);
}
