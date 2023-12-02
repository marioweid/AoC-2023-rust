use core::num;
use std::collections::btree_map::Values;
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

fn import_game_data(row: &str) -> GamesData{
    // // File hosts.txt must exist in the current path
    // if let Ok(lines) = read_lines("./src/example.txt") {
    //     // Consumes the iterator, returns an (Optional) String
    //     let mut game_map = HashMap::from()
    //     for line in lines {
    //         if let Ok(ip) = line {

    //             println!("{}", ip);
    //         }
    //     }
    // }

    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    let mut ret: HashMap<String, HashMap<String, Vec<u32>>> = GamesData::new();
    let mut splitted: std::str::Split<'_, &str> = row.split(": ");
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
    ret
}

fn create_color_counts(pairs: Vec<(&str, u32)>) -> ColorCounts {
    let mut color_counts: HashMap<String, Vec<u32>> = ColorCounts::new();
    for (color, count) in pairs {
        // get entry or create new one
        let entry: &mut Vec<u32> = color_counts.entry(color.to_string()).or_insert(Vec::new());
        // push value to the entry
        entry.push(count);

    }
    color_counts
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

    let row = "Game 3: 8 green, 6 blue, 10 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let color_tuples: HashMap<&str, u32> = HashMap::from_iter([("red", 12u32),("green", 13u32),("blue", 14u32)]);
    let game_row: HashMap<String, HashMap<String, Vec<u32>>> = import_game_data(row);
    let is_valid: bool = is_valid_row(game_row.get("Game 3"), &color_tuples);

    println!("is_valid: {}", is_valid);    
    println!("Breakpoint");
    
    // ------------------------
    let mut games_data: GamesData = HashMap::new();
    games_data.insert("Game 1".to_string(), create_color_counts(vec![("blue", 3), ("red", 4), ("red", 1), ("green", 2), ("blue", 6), ("green", 2)]));
    games_data.insert("Game 2".to_string(), create_color_counts(vec![("blue", 1), ("green", 2), ("green", 3), ("blue", 4), ("red", 1), ("green", 1), ("blue", 1)]));
    games_data.insert("Game 3".to_string(), create_color_counts(vec![("green", 8), ("blue", 6), ("red", 20), ("blue", 5), ("red", 4), ("green", 13), ("green", 5), ("red", 1)]));
    games_data.insert("Game 4".to_string(), create_color_counts(vec![("green", 1), ("red", 3), ("blue", 6), ("green", 3), ("red", 6), ("green", 3), ("blue", 15), ("red", 14)]));
    games_data.insert("Game 5".to_string(), create_color_counts(vec![("red", 6), ("blue", 1), ("green", 3), ("blue", 2), ("red", 1), ("green", 2)]));


    // Check if there is a match for a given condition

    let mut valid_rows: Vec<i32> = Vec::new();
    for i in 1..6 {
        let row_index: String = format!("Game {}", i);
        let row: Option<&HashMap<String, Vec<u32>>> = games_data.get(row_index.as_str());
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



    // ------------------------




    // if let Some(color_counts) = games_data.get(game_name) {
    //     let actual_colors_counts = color_counts.get(color_to_check);
        
    //     let col_match = match actual_colors_counts {
    //         Some(colors) => { if colors.iter().all(|f| f <= &count_to_check){
    //             println!("Possible: {}", color_to_check);
    //             true
    //         }
    //         else{
    //             println!("Impossible: {}", color_to_check);
    //             false
    //         }
    //     }
    //         None => {println!("Invalid: colors empty {}.", game_name); false}
    //     };
    //     println!("col_match: {}", col_match)




    //     // if Some(actual_colors_counts) {
    //     //     if actual_colors_counts.iter().all(|val| val < &count_to_check) {
    //     //         println!("Valid: {} has exactly {} {}.", game_name, count_to_check, color_to_check);
    //     //     } else {
    //     //         println!("Invalid: {}.", game_name);
    //     //     }
    //     // } else {
    //     //     println!("Invalid: {} does not contain any {}.", game_name, color_to_check);
    //     // }
    // } else {
    //     println!("Invalid: {} not found.", game_name);
    // }


}
