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
        let mut card_counts: HashMap<&String, i32> = std::collections::HashMap::new();

        for card in self.cards.iter() {
            let count: &mut i32 = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        
        let count_vals: Vec<i32> = card_counts.values().cloned().collect();
        let mut _ret: Option<&str> = None;
        if count_vals.contains(&5) {_ret = Some("five_oak")}
        else if count_vals.contains(&5) { _ret = Some("four_oak") }
        else if count_vals.contains(&4) { _ret = Some("four_oak") }
        else if count_vals.contains(&3) & count_vals.contains(&2) { _ret = Some("full_house") }
        else if count_vals.contains(&3) { _ret = Some("three_oak") }
        else if count_vals.iter().filter(|&val| *val==2).count() == 2 { _ret = Some("two_pair") }
        else if count_vals.contains(&2) { _ret = Some("one_pair") }
        else if count_vals.iter().all(|val| *val==1) { _ret = Some("high_card") }
        else { _ret = Some("rank") }
        
        return _ret.unwrap().to_string()
    }

    fn determine_hand_power(&mut self) -> i32 {
        let mapping: HashMap<&str, i32> = HashMap::from([
            ("A", 14), ("K", 13), ("Q", 12), ("J", 11), ("T", 10), ("9", 9),
            ("8", 8),("7", 7), ("6", 6), ("5", 5), ("4", 4), ("3", 3), ("2", 2)]);
        let power: i32 = self.cards.iter().map(|val| mapping.get(val.as_str()).unwrap()).sum();
        return power;
    }
}


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

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
fn main() {
    let hands:Vec<Hand>  = read_data("../example.txt");

    println!("Amount of Hands: {}", hands.len());
    for hand in &hands{
        println!("power: {}", hand.power);
    }
    println!("Breakpoint!")
}
