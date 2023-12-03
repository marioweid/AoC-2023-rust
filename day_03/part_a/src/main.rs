use regex::Regex;
use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


    
fn padding_grid(mut lines: Vec<String>) -> Vec<String> {

    // How long is the first line (assuming all lines are same length)

    let length = lines.first().unwrap().len();
    
    // Create an artificial line full of dots of same length
    let dotstring = String::from(".").repeat(length);
    
    // Add to top
    lines.insert(0, dotstring.clone());

    // Add to bottom
    lines.push(dotstring.clone());


    for line in lines.iter_mut() {
        line.insert(0, '.');
        line.push('.');
    }

    lines
}


fn check_for_symbol(s: &str) -> bool {
    let pattern = Regex::new(r"[^\d\.]").unwrap();

    return pattern.find(s).is_some()
}

fn main() {
    // Create a regex pattern

    let lines = read_lines("../input.txt");

    let padded_lines = padding_grid(lines);

    let pattern = Regex::new(r"\d+").unwrap();

    let mut engine_parts: Vec<i32> = Vec::new();

    for (line_idx, line) in padded_lines.iter().enumerate() {
    
        println!("Line number {}", &line_idx);

        // Use find_iter to iterate over matches
        for mat in pattern.find_iter(line) {
            // Access the matched substring
            let number = mat.as_str().parse::<i32>().unwrap();
    
            let start: usize = mat.start();
            let end: usize = mat.end();
    
            println!("\tFound number: {}", &number);

            // Extend the span by -1 / +1
            let extended_start = &start -1;
            let extended_end = &end + 1;

             // Get relevant part of prev/current/next line
            let range = extended_start..extended_end;
            
            let prev_slice = &padded_lines[&line_idx -1][range.clone()];
            let current_slice = &padded_lines[line_idx.clone()][range.clone()];
            let next_slice = &padded_lines[&line_idx +1][range.clone()];


            let engine_part_vector: [bool; 3] = [
                check_for_symbol(prev_slice),
                check_for_symbol(current_slice),
                check_for_symbol(next_slice)
            ];


            let is_engine_part = engine_part_vector.iter().any(|val: &bool| *val == true);

            println!("Is engine part: {}", &is_engine_part );


            if is_engine_part{
                engine_parts.push(number);
            }

    }

    let result_sum:i32= engine_parts.iter().sum();


    println!("Result: {}", result_sum);

    }
}