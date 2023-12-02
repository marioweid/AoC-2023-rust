use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

type ColorCounts = HashMap<String, Vec<u32>>;
type GamesData = HashMap<String, ColorCounts>;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_as_game_data() -> GamesData{
    
    // File hosts.txt must exist in the current path
    let mut ret: HashMap<String, HashMap<String, Vec<u32>>> = GamesData::new();
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(line) = line_result {
                // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                let mut splitted: std::str::Split<'_, &str> = line.as_str().split(": ");
                // Just yolo unwrap the values 
                // Game 3
                let game_key: &str = splitted.next().unwrap();
                // 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                let game_string: &str = splitted.next().unwrap();
                // 8 green, 6 blue, 20 red
                // 5 blue, 4 red, 13 green; 5 green, 1 red
                let splitted_sets: Vec<&str> = game_string.split("; ").into_iter().collect();
                for set in splitted_sets {
                    let color_tuples: std::str::Split<'_, &str> = set.split(", ");
                    for tuple in color_tuples {
                        let mut values: std::str::Split<'_, &str> = tuple.split(" ");
                        let digit = values.next().unwrap().parse::<u32>().unwrap();
                        let color_key = values.next().unwrap();

                        let game_entry: &mut HashMap<String, Vec<u32>> = ret.entry(game_key.to_string()).or_insert(HashMap::new());
                        let color_entry: &mut Vec<u32> = game_entry.entry(color_key.to_string()).or_insert(Vec::new());
                        color_entry.push(digit)
                    }
                }
            } else {
                println!("yolo");
            }
        }
    }
    else {
        println!("yolo²");
    }
    return ret;
}

fn is_valid_row(row: Option<&ColorCounts>, possible_values: &HashMap<&str, u32>) -> bool{
    // check if the row exists 
    let ret: bool = match row {
        Some(row_values) => {
            let all_smaller = row_values.iter().all(|(key, values)| {
                return possible_values.get(key.as_str()).map_or(false, |possible_val| {
                    values.iter().all(|val| val <= possible_val) 
                });
            });
            all_smaller
        }   
        None => {
            false
        }     
    };
    ret
}

fn game_string_to_int(game_string: String) -> i32 {
    let parse_result: Option<Result<i32, num::ParseIntError>> = game_string
        .split_whitespace()
        .last()
        .map(|val| val.parse::<i32>());

    match parse_result {
        Some(Ok(number)) => number,
        _ => 0,
    }
}

fn main() {
    let color_tuples: HashMap<&str, u32> = HashMap::from_iter([("red", 12u32),("green", 13u32),("blue", 14u32)]);
    let game_data_map: HashMap<String, HashMap<String, Vec<u32>>> = read_file_as_game_data();

    // check every entry in the game map and safe the valid entries
    let mut valid_rows: Vec<i32> = Vec::new();
    for i in 0..game_data_map.len() {
        let row_index: String = format!("Game {}", i + 1);
        let row: Option<&HashMap<String, Vec<u32>>> = game_data_map.get(row_index.as_str());
        let is_valid: bool = is_valid_row(row, &color_tuples);
        if is_valid {
            valid_rows.push(game_string_to_int(row_index.clone()));
        }
    }

    for index in &valid_rows {
        println!("{}", index);
    }

    let the_sum: i32 = valid_rows.into_iter().sum();
    println!("The sum: {}", the_sum);
}
