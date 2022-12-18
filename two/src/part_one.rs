pub(crate) fn part_one(lines: &[&str]) -> i32 {
    let win = 6;
    let draw = 3;
    let loss = 0;

    let mut sum = 0;
    for line in lines {
        // split line into two teams
        let teams = line.split(' ').collect::<Vec<&str>>();
        let (team_one, team_two) = (teams[0], teams[1]);
        let score = match (team_one, team_two) {
            ("A", "X") => draw + 1,
            ("A", "Y") => win + 2,
            ("A", "Z") => loss + 3,
            ("B", "X") => loss + 1,
            ("B", "Y") => draw + 2,
            ("B", "Z") => win + 3,
            ("C", "X") => win + 1,
            ("C", "Y") => loss + 2,
            ("C", "Z") => draw + 3,
            _ => panic!("Invalid input"),
        };
        sum += score;
    }
    sum
}

// test part_one
#[test]
pub(crate) fn test_part_one() {
    let lines = "A Y\nB X\nC Z".split('\n').collect::<Vec<&str>>();
    assert_eq!(part_one(&lines), 15);
}
