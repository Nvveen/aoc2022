mod part_one;
mod part_two;
mod shared;

fn main() {
    let input = &include_str!("./input");
    let part_one = part_one::part_one(input);
    println!("Part one: {}", part_one);

    let part_two = part_two::part_two(input);
    println!("Part two: {}", part_two);
}
