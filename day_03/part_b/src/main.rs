use regex::{Regex, Matches, Match};
use core::{num, slice};
use std::{fs::read_to_string, path::Iter, collections::HashMap, panic::AssertUnwindSafe};


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


fn find_gears(s: &str) ->  Vec<Match>{
    let pattern = Regex::new(r"[*]").unwrap();

    let found_gears = pattern.clone().find_iter(s).collect();

    return found_gears
}


fn main() {
    // Create a regex pattern

    let lines = read_lines("../input.txt");

    let padded_lines = padding_grid(lines);

    let pattern = Regex::new(r"\d+").unwrap();

    let mut gear_map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    // (line, column) : [number1, number2]
    // (line, column) : [number3]

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


            let slices_array: [(i32, &str); 3] = [
                (-1, prev_slice),
                (0, current_slice),
                (1, next_slice)
                ];

            for (modifier, slice) in slices_array {

                // Handle all found gears in previous slice:
                let gears_slice = find_gears(slice);
                for mat in gears_slice {
                    
                    // Coorindates for found gear
                    let gear_column = mat.start() + &extended_start;
                    let gear_position = ((line_idx as i32 + modifier), gear_column as i32);
                    
                    println!("Result: ");

                    let entry = gear_map.entry(gear_position).or_insert(Vec::new());

                    entry.push(number.clone());

            }

        }

    }




    
}

let mut results: Vec<i32> = Vec::new();

for (_position, numbers) in gear_map.iter() {

    if numbers.len() as i32 > 1{
        results.push(numbers[0] * numbers[1]);

    }
}

let final_sum: i32= results.into_iter().sum();

println!("Final sum: {}", final_sum);

}