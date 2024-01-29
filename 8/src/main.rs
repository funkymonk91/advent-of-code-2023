use std::fs;

fn main() {
    let file_path = "src/input.txt";
    // let file_path = "src/test.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    println!("Part 1: {}", part_1(&file_lines));
}

fn part_1 (file_lines: &Vec<&str>) -> i32 {
    0
}