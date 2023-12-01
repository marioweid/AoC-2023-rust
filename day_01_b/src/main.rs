use std::fs::File;
use std::io;
use std::io::BufRead;

fn string_sum(s: &str) -> i32 {
    let first_number = s.chars().find(|c| c.is_numeric());
    let last_number = s.chars().rev().find(|c| c.is_numeric());

    // is a number
    match (first_number, last_number) {
        (Some(first), Some(last)) => {
            let first_num = first.to_digit(10).unwrap_or(0) as i32;
            let last_num = last.to_digit(10).unwrap_or(0) as i32;
            format!("{}{}", first_num, last_num).parse().unwrap_or(0)
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

fn map_string_to_number(s: Option<&str>) -> i32 {
    match s {
        Some("one") => 1,
        Some("two") => 2,
        Some("three") => 3,
        Some("four") => 4,
        Some("five") => 5,
        Some("six") => 6,
        Some("seven") => 7,
        Some("eight") => 8,
        Some("nine") => 9,
        Some("1") => 1,
        Some("2") => 2,
        Some("3") => 3,
        Some("4") => 4,
        Some("5") => 5,
        Some("6") => 6,
        Some("7") => 7,
        Some("8") => 8,
        Some("9") => 9,
        _ => 0, // or handle the case when the string doesn't match any known number
    }
}

fn get_line_sum(line: String, strings_to_match: Vec<&str>) -> (i32, i32){
    println!("Trying: {}",line.as_str());
    let mut first_match: Option<&str> = None;
    let mut last_match: Option<&str> = None;

    for (i, c) in line.chars().enumerate() {
        let sub_string: String = line.chars().take(i).collect();
        let rev_string: String = line.chars().rev().take(i).collect();

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
    (map_string_to_number(first_match), map_string_to_number(last_match))
}

fn main() {
    let valid_chars = vec!["1","2","3","4", "5", "6", "7", "8", "9", "one",
                           "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let full_string: String = String::from("zoneight234");
    let (a, b) = get_line_matches(full_string,valid_chars);
    eprintln!("a: {} b: {}",a, b);
    //
    // match read_file_lines("./src/example.txt"){
    //     Ok(lines) => {
    //         let mut first_match: Option<&str> = None;
    //         let mut last_match: Option<&str> = None;
    //         for line in lines {
    //             for (i, c) in line.chars().enumerate() {
    //                 let norm_sub_string: String = line.chars().take(i).collect();
    //                 let rev_sub_string: String = line.chars().rev().take(i).collect();
    //                 println!("{}", norm_sub_string);
    //
    //                 for v_char in &valid_chars{
    //                     if norm_sub_string.contains(v_char){
    //                         first_match = Some(v_char);
    //                         break;
    //                     }
    //                     if rev_sub_string.contains(v_char){
    //                         last_match = Some(v_char);
    //                         break;
    //                     }
    //
    //                     // Check if both first and last are set, then break out of the loop
    //                     if first_match.is_some() && last_match.is_some() {
    //                         break;
    //                     }
    //                 }
    //                 // Check if both first and last are set, then break out of the outer loop
    //                 if first_match.is_some() && last_match.is_some() {
    //                     break;
    //                 }
    //             }
    //             // Print or use the values of first and last outside the loop
    //             if let (Some(first_val), Some(last_val)) = (first_match, last_match) {
    //                 println!("First: {}, Last: {}", first_val, last_val);
    //             }
    //         }
    //
    //     }
    //     Err(e) => eprintln!("Error reading file: {}", e),
    // }
}
