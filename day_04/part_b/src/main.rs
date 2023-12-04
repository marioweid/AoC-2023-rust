use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type NumberCombination = HashMap<String, Vec<i32>>;
type Scratchcards = HashMap<i32, NumberCombination>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_as_scratch_cards(filepath: &str) -> Scratchcards {
    // File hosts.txt must exist in the current path
    let mut ret: HashMap<i32, HashMap<String, Vec<i32>>> = Scratchcards::new();

    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(line) = line_result {
                // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                let mut splitted: std::str::Split<'_, &str> = line.as_str().split(": ");
                // Just yolo unwrap the values
                let card_key: &str = splitted.next().unwrap(); // Card 1
                let card_key = card_key.split_whitespace().last().unwrap().parse::<i32>().unwrap(); // Card1

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

                let card_entry: &mut HashMap<String, Vec<i32>> = ret.entry(card_key).or_insert(HashMap::new());
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


// 1. Für jede Karte im Original
// 1. Berechne Matches und adde matches
// 2. für jedes Match => Berechne Matches
// 2. Summe über alle matches
fn get_rec_card_indexes(card_stack: &HashMap<i32, HashMap<String, Vec<i32>>>, current_card_key: &i32) ->  Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut card_tree: Vec<i32> = Vec::new();
    let card_entry: &HashMap<String, Vec<i32>> = card_stack.get(current_card_key).unwrap();
    
    let actual: &Vec<i32> = card_entry.get("actual").unwrap();
    let winners: &Vec<i32> = card_entry.get("winning").unwrap();
    let correct_numbers: Vec<i32> = actual
    .iter()
    .filter(|&x| winners.contains(x))
    .cloned()
    .collect();
    let amount_correct: i32 = correct_numbers.len() as i32;
    let max_return_index = current_card_key + current_card_key + amount_correct;
    
    for i in *current_card_key+1..max_return_index{
        let max_index = card_stack.len() as i32;
        if i < max_index{
            println!("Adding: {}", &i);
            card_tree.push(i);
            ret.push(i);
        }

    }

    for key in &card_tree{
        get_rec_card_indexes(card_stack,&key);
    }
    ret
}

fn main() {
    let card_stack: HashMap<i32, HashMap<String, Vec<i32>>> =
        read_file_as_scratch_cards("../example.txt");

    let mut owned_cards: Vec<HashMap<String, Vec<i32>>> = Vec::new();
    let mut ret: Vec<i32> = Vec::new();
    for card in &card_stack {
        let mut tmp: Vec<i32> = get_rec_card_indexes(&card_stack, card.0);
    }
}
