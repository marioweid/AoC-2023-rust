use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn create_color_counts(pairs: Vec<(&str, u32)>) -> ColorCounts {
    let mut color_counts = ColorCounts::new();
    for (color, count) in pairs {
        // get entry or create new one
        let entry = color_counts.entry(color.to_string()).or_insert(Vec::new());
        // push value to the entry
        entry.push(count);

    }
    color_counts
}

type ColorCounts = HashMap<String, Vec<u32>>;
type GamesData = HashMap<String, ColorCounts>;

fn main() {
    let mut games_data: GamesData = HashMap::new();

    games_data.insert("Game 1".to_string(), create_color_counts(vec![("blue", 3), ("red", 4), ("red", 1), ("green", 2), ("blue", 6), ("green", 2)]));
    games_data.insert("Game 2".to_string(), create_color_counts(vec![("blue", 1), ("green", 2), ("green", 3), ("blue", 4), ("red", 1), ("green", 1), ("blue", 1)]));
    games_data.insert("Game 3".to_string(), create_color_counts(vec![("green", 8), ("blue", 6), ("red", 20), ("blue", 5), ("red", 4), ("green", 13), ("green", 5), ("red", 1)]));


    // Check if there is a match for a given condition
    let game_name = "Game 1";
    let color_to_check = "blue";
    let count_to_check = 12u32;

    if let Some(color_counts) = games_data.get(game_name) {
        let actual_colors_counts = color_counts.get(color_to_check);
        
        let col_match = match actual_colors_counts {
            Some(colors) => { if colors.iter().all(|f| f <= &count_to_check){
                println!("Possible: {}", color_to_check);
                true
            }
            else{
                println!("Impossible: {}", color_to_check);
                false
            }
        }
            None => {println!("Invalid: colors empty {}.", game_name); false}
        };
        println!("col_match: {}", col_match)

        // if Some(actual_colors_counts) {
        //     if actual_colors_counts.iter().all(|val| val < &count_to_check) {
        //         println!("Valid: {} has exactly {} {}.", game_name, count_to_check, color_to_check);
        //     } else {
        //         println!("Invalid: {}.", game_name);
        //     }
        // } else {
        //     println!("Invalid: {} does not contain any {}.", game_name, color_to_check);
        // }
    } else {
        println!("Invalid: {} not found.", game_name);
    }

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
}
