use std::collections::VecDeque;

fn main() {
    // read input
    let input = &include_str!("input");

    // part one
    let part_one = part_one(input);
    println!("Part one: {}", part_one);
}

fn part_one(input: &str) -> String {
    let stacks = input
        .lines()
        .filter(|line| !line.is_empty() || line.starts_with(" 1 "))
        .fold(Vec::new(), |stacks, line| match line {
            line if line.starts_with("move") => move_crates(line, stacks),
            _ => create_chunks(line, stacks),
        });
    // get the last item of every stack and turn it into a string
    stacks
        .into_iter()
        .map(|stack| *stack.back().unwrap())
        .collect::<String>()
}

/**
 * Moves crates from one stack to another. The line is expected to be in the
 * format "move <count> from <from> to <to>".
 */
fn move_crates(line: &str, mut stacks: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let items: Vec<u32> = line
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, s)| s.parse().unwrap())
        .collect();
    let (count, from, to) = (items[0], items[1] - 1, items[2] - 1);
    let stack: &mut VecDeque<char> = stacks.get_mut(from as usize).unwrap();
    let crates = stack.split_off(stack.len() - count as usize);
    let crates = crates.into_iter().rev().collect::<VecDeque<char>>();
    stacks.get_mut(to as usize).unwrap().extend(crates);
    stacks
}

/**
 * Create new stacks per line. The line is expected to be in the format
 * "[A] [B] [C]".
 */
fn create_chunks(line: &str, mut stacks: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    // Loop per 4 characters (chunk) and push to the stack.
    for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
        let c = chunk[1] as char;
        if i >= stacks.len() {
            stacks.push(VecDeque::new());
        }
        if c.is_alphanumeric() {
            // push to front of stack
            stacks.get_mut(i).unwrap().push_front(c);
        }
    }
    stacks
}

#[test]
fn test_part_one() {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part_one(input), "CMZ");
}
