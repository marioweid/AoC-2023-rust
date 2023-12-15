use std::fs;


fn read_file(file_path: &str) -> Vec<Vec<&str>>{
    let output: Vec<Vec<&str>> = Vec::new();
    let content = fs::read_to_string(file_path)
    .expect("Should have been able to read the file"); // TODO Split by \n
    println!("{:?}", content);
    output
}

pub fn main() {
    let content = read_file("../example.txt");
    println!("Day 10 Solution A!");
}
