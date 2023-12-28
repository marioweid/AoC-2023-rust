use std::fs;

fn read_file(file_path: &str) {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    // TODO
    return 0;
}

pub fn main() {
    let total :i32 = 0;
    println!("Result: {}", total);
}
