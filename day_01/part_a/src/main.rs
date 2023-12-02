use std::fs::File;
use std::io;
use std::io::BufRead;

fn string_sum(s: &str) -> i32 {
    let first_numeric = s.chars().find(|c| c.is_numeric());
    let last_numeric = s.chars().rev().find(|c| c.is_numeric());

    // is a number
    match (first_numeric, last_numeric) {
        (Some(first), Some(last)) => {
            format!("{}{}", first, last).parse().unwrap_or(0)
        }
        _ => 0,
    }
}

fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

fn main() {
    match read_file_lines("../example.txt") {
        Ok(lines) => {
            let total_sum: i32 = lines
                .iter()
                .map(|line| string_sum(line.as_str()))
                .sum();

            println!("Total Sum: {}", total_sum);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
