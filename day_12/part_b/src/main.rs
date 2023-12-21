use std::fs;

fn read_file(file_path: &str) -> (Vec<char>, Vec<i32>) { //TODO REPLACE FUNCTION
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    return grid;
}

pub fn main() {
    println!("Part B")
}
