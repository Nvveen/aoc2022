mod part_one;
mod part_two;

fn main() {
    // read the file
    let contents = std::fs::read_to_string("./three/src/input")
        .expect("Should have been able to read the file");

    println!("Part one: {}", part_one::part_one(&contents));

    println!("Part two: {}", part_two::part_two(&contents));
}
