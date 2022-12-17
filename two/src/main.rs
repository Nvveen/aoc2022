use std::{collections::HashMap, fs};

static FILENAME: &str = "./two/src/input";

fn main() {
    // read the file
    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the file");
    // let contents = "A Y\nB X\nC Z";

    // split the input into lines
    let lines = contents.split('\n');

    // The score for every matchup against A, B, C
    let score_lookup = [[3, 0, 6], [6, 3, 0], [0, 6, 3]];

    // Precalculate the score
    let score_map = (0..3)
        .flat_map(|i| {
            (0..3).map(move |j| {
                let opp = ["A", "B", "C"][j];
                let player = ["X", "Y", "Z"][i];
                let score = score_lookup[i][j] + i as i32 + 1;
                ((opp, player), score)
            })
        })
        .collect::<HashMap<_, _>>();

    let total_score: i32 = lines
        .map(|line| {
            // split line into parts
            let parts = line.split(' ').collect::<Vec<_>>();
            let (opp, player) = (parts[0], parts[1]);
            *score_map.get(&(opp, player)).unwrap_or(&0)
        })
        .sum();

    // print score
    println!("Total score: {}", total_score);
}
