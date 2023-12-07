#![allow(dead_code)]
use std::{fs, collections::HashMap};

struct Hand {
    cards: Vec<String>,
    bid: i32,
    power: i32,
    rank: Option<i32>,
    hand_type: String
}

impl Hand {

    fn new(cards: Vec<String>, bid: i32) -> Hand {
        let mut instance : Hand = Hand {
            cards: cards,
            bid: bid,
            power: 0,
            hand_type: "".to_string(),
            rank: None,
        };
        instance.power = Self::determine_hand_power(&mut instance);
        instance.hand_type = Self::determine_hand_type(&mut instance);
        return instance
    }

    fn determine_hand_type(&mut self) -> String {
        return "".to_string()
    }

    fn determine_hand_power(&mut self) -> i32 {
        let mapping: HashMap<&str, i32> = HashMap::from([
            ("A", 14), ("K", 13), ("Q", 12), ("J", 11), ("T", 10), ("9", 9),
            ("8", 8),("7", 7), ("6", 6), ("5", 5), ("4", 4), ("3", 3), ("2", 2)]);
        let power: i32 = self.cards.iter().map(|val| mapping.get(val.as_str()).unwrap()).sum();
        return power;
    }
}

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
fn read_data(filepath: &str) -> Vec<Hand> {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    let lines = data.split("\n");
    
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines{
        let mut line_split = line.split_ascii_whitespace();
        
        //Cards
        let cards: Vec<String> = line_split
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|val| val.to_string()).collect();
    
        // Bids
        let bid: i32 = line_split
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

        hands.push(Hand::new(cards, bid));
    }
    

    return hands;
}

fn main() {
    let hands:Vec<Hand>  = read_data("../example.txt");

    println!("Amount of Hands: {}", hands.len());
    for hand in hands{
        println!("power: {}", &hand.power);
    }
}
