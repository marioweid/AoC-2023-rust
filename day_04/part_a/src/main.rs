use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type NumberCombination = HashMap<String, Vec<i32>>;
type Scratchcards = HashMap<String, NumberCombination>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_as_scratch_cards(filepath: &str) -> Scratchcards {
    // File hosts.txt must exist in the current path
    let mut ret: HashMap<String, HashMap<String, Vec<i32>>> = Scratchcards::new();

    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(line) = line_result {
                // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                let mut splitted: std::str::Split<'_, &str> = line.as_str().split(": ");
                // Just yolo unwrap the values
                let card_key: &str = splitted.next().unwrap(); // Card 1
                let card_key = card_key.replace(" ", ""); // Card1

                let scratch_numbers: &str = splitted.next().unwrap(); // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                                                                      

                let mut splitted_sets = scratch_numbers.split(" | ").into_iter();

                let winning_numbers: Vec<i32> = splitted_sets.next().unwrap()// 41 48 83 86 17
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                let actual_numbers: Vec<i32> = splitted_sets.next().unwrap() // 83 86  6 31 17  9 48 53
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                let card_entry: &mut HashMap<String, Vec<i32>> = ret.entry(card_key.to_string()).or_insert(HashMap::new());
                card_entry.insert("winning".to_string(), winning_numbers);
                card_entry.insert("actual".to_string(), actual_numbers);
            } else {
                println!("yolo");
            }
        }
    } else {
        println!("yolo²");
    }
    return ret;
}

fn get_winning_score(winning_numbers: &Vec<i32>, actual_numbers: &Vec<i32>)-> i32 {
    let common_elements: Vec<i32> = actual_numbers
    .iter()
    .filter(|&x| winning_numbers.contains(x))
    .cloned()
    .collect();

    if common_elements.is_empty(){
        return 0;
    }
    else {
        let power: u32 = common_elements.len() as u32 - 1;
        return 2i32.pow(power)
    }

}

fn main() {
    let scratch_card_map: HashMap<String, HashMap<String, Vec<i32>>> =
        read_file_as_scratch_cards("../input.txt");

    let mut scores: Vec<i32> = Vec::new();
    for i in 0..scratch_card_map.len(){
        let card_key = format!("Card{}", i+1);
        println!("card_key: {}", &card_key);
        let row = scratch_card_map.get(&card_key).unwrap();
        let winning = row.get("winning").unwrap();
        let actual = row.get("actual").unwrap();
        let score = get_winning_score(winning, actual);
        scores.push(score);
    }
    let total_score: i32 = scores.iter().sum();
    println!("Total score: {}", total_score);
}
