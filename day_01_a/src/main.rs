use std::fs::{File, read_to_string};
use std::io;
use std::io::BufRead;
use std::path::Path;

#[allow(dead_code)]
fn read_lines_in_memory(filename: &str) -> Vec<String>{
    let mut result :Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

// Better version that reads with buffer not creates a string in buffer
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./src/example.txt"){
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip)
            }
        }
    }
}
