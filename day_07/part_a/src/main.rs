#![allow(dead_code)]
use std::{fs, collections::HashMap, cmp::Ordering};

#[derive(Debug, Eq, PartialEq)]
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
        let card_count: i32 = self.cards.len() as i32;
        let power: i32 = self.cards.iter().enumerate().map(|(index, val)| mapping.get(val.as_str()).unwrap() * (20i32.pow((card_count as u32) - (index as u32)))).sum();
        return power;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare based on type
        let type_ordering: Ordering = match (self.hand_type.as_str(), other.hand_type.as_str()) {
            ("five_oak", "five_oak") => Ordering::Equal,
            ("five_oak", _) => Ordering::Greater,
            (_, "five_oak") => Ordering::Less,
            ("four_oak", "four_oak") => Ordering::Equal,
            ("four_oak", _) => Ordering::Greater,
            (_, "four_oak") => Ordering::Less,
            ("full_house", "full_house") => Ordering::Equal,
            ("full_house", _) => Ordering::Greater,
            (_, "full_house") => Ordering::Less,
            ("three_oak", "three_oak") => Ordering::Equal,
            ("three_oak", _) => Ordering::Greater,
            (_, "three_oak") => Ordering::Less,
            ("two_pair", "two_pair") => Ordering::Equal,
            ("two_pair", _) => Ordering::Greater,
            (_, "two_pair") => Ordering::Less,
            ("one_pair", "one_pair") => Ordering::Equal,
            ("one_pair", _) => Ordering::Greater,
            (_, "one_pair") => Ordering::Less,
            ("rank", "rank") => Ordering::Equal,
            ("rank", _) => Ordering::Greater,
            (_, "rank") => Ordering::Less,
            // Continue with other type comparisons...
            _ => self.hand_type.cmp(&other.hand_type),
        };

        // If types are equal, compare based on rank
        if type_ordering == Ordering::Equal {
            self.power.cmp(&other.power)
        } else {
            type_ordering
        }
    }    
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

fn main() {
    let mut hands: Vec<Hand>  = read_data("../input.txt");
    hands.sort();

    for (index, mut hand) in hands.iter_mut().enumerate(){
        hand.rank = Some(index as i32 + 1) ;
    }

    let total_winnings: i32 = hands
    .iter()
    .map(|h| h.bid * h.rank.unwrap())
    .sum();

    println!("total_winnings: {}", total_winnings);
}
