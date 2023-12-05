use std::collections::HashMap;
use std::fs;


struct SourceDestinationEntry {
    source: Vec<i64>,
    destination: Vec<i64>,
}

impl SourceDestinationEntry {
    fn new(dest_start: i64, source_start: i64, range: i64) -> SourceDestinationEntry {
        SourceDestinationEntry {
            source: (source_start..source_start + range).collect(),
            destination: (dest_start..dest_start + range).collect(),
        }
    }

    fn to_hashmap(&self) -> HashMap<i64, i64> {
        let ret: HashMap<i64, i64> = self
            .source
            .iter()
            .cloned()
            .zip(self.destination.clone().into_iter())
            .collect();
        return ret;
    }
}

fn combine_entries(entries: Vec<SourceDestinationEntry>) -> HashMap<i64, i64> {
    return entries
        .into_iter()
        .flat_map(|entry| entry.to_hashmap())
        .collect();
}

#[derive(Clone)]
struct Map {
    _key: String,
    mapping: HashMap<i64, i64>,
    next_map: Option<Box<Map>>, // Optional next map
}


impl Map {
    pub fn new(key: &str, mapping_entries: Vec<SourceDestinationEntry>) -> Map {
        Map {
            _key: String::from(key),
            mapping: combine_entries(mapping_entries),
            next_map: None,
        }
    }

    pub fn set_next_map(&mut self, next_map: Map) {
        self.next_map = Some(Box::new(next_map));
    }

    pub fn source_to_destination(&self, source_value: i64) -> i64 {
        let dest = self.mapping.get(&source_value).unwrap_or(&source_value);

        match &self.next_map {
            Some(map) => {
                let recursive_result = map.source_to_destination(*dest);
                recursive_result
            }
            None => *dest,
        }
    }
}

fn read_data(filepath: &str) -> (Vec<i64>, Map) {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    let mut paragraphs = data.split("\n\n");
    let seeds: Vec<i64> = paragraphs
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut maps:Vec<Map> = Vec::new();

    for paragraph in paragraphs {
        let mut key_value_split = paragraph.split(" map:\n");
        let key = key_value_split.next().unwrap(); // seed-to-soil map:

        let values: Vec<&str> = key_value_split.next().unwrap().split("\n").collect(); // 50 98 2  -  52 50 48
        let value_array: Vec<Vec<i64>> = values.iter()
        .map(|s| s.split_whitespace().map(|num| num.parse().unwrap()).collect())
        .collect();
        let mut source_dest_entries: Vec<SourceDestinationEntry> = Vec::new();
        
        for array in value_array{
            let entry = SourceDestinationEntry::new(
                array[0], array[1], array[2]);
                source_dest_entries.push(entry);

        }
        let map: Map = Map::new(key, source_dest_entries);
        maps.push(map);
    }

    for idx in (0..maps.len() - 1).rev(){
        let next: Map = maps[idx+1].clone();
        maps[idx].set_next_map(next)
    }
    return (seeds, maps[0].clone()); // return seeds and root element
}

fn main() {
    let (seeds,map)  = read_data("../input.txt");
    let mut results :Vec<i64> = Vec::new();
    for seed in seeds{
        let res = map.source_to_destination(seed);
        println!("result for seed {}->{}", seed, &res);
        results.push(res);
    }
    let shortest = results.iter().min().unwrap();
    println!("Shortest Seed: {}",shortest);
}
