use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(file_path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

fn map_string_to_number(s: Option<&str>) -> &str {
    match s {
        Some("one") => "1",
        Some("two") => "2",
        Some("three") => "3",
        Some("four") => "4",
        Some("five") => "5",
        Some("six") => "6",
        Some("seven") => "7",
        Some("eight") => "8",
        Some("nine") => "9",
        Some("1") => "1",
        Some("2") => "2",
        Some("3") => "3",
        Some("4") => "4",
        Some("5") => "5",
        Some("6") => "6",
        Some("7") => "7",
        Some("8") => "8",
        Some("9") => "9",
        _ => "0", // or handle the case when the string doesn't match any known number
    }
}

fn get_line_square_sum(line: String) -> i32{
    let strings_to_match: Vec<&str>= vec!["1","2","3","4", "5", "6", "7", "8", "9", "one",
                                     "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut first_match: Option<&str> = None;
    let mut last_match: Option<&str> = None;

    for (i, _c) in line.chars().enumerate() {
        let sub_string: String = line.chars().take(i + 1).collect();
        let rev_string: String = line.chars().skip(line.len() - i - 1).collect();
        println!("{}", sub_string);
        println!("{}", rev_string);
        for candidate in &strings_to_match{
            if first_match.is_none() & sub_string.contains(candidate) {
                first_match = Some(candidate);
            }
            if last_match.is_none() & rev_string.contains(candidate){
                last_match = Some(candidate);
            }
        }

        if first_match.is_some() & last_match.is_some(){
            // break condition, both numbers got matched
            (map_string_to_number(first_match), map_string_to_number(last_match));
        }
    }
    format!("{}{}",
            map_string_to_number(first_match),
            map_string_to_number(last_match))
        .parse()
        .unwrap_or(0)
}

fn main() {
    match read_file_lines("../example.txt"){
        Ok(lines) => {
            let mut line_sums : Vec<i32> = Vec::new();
            for line in lines {
                let a = get_line_square_sum(line);
                println!("sum: {}",a);
                println!("len: {}",line_sums.len());
                line_sums.push(a);
            }
            let total: i32 = line_sums.iter().sum();
            println!("Total: {}", total);
            println!("len: {}", line_sums.len());
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
