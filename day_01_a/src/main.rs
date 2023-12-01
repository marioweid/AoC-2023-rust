use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String>{
    let mut result :Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

fn main() {
    let mut lines: Vec<String> = read_lines("./src/example.txt");
    for line in lines{
        println!("{}", line);
    }
}
