use std::collections::HashMap;
use std::hash::Hash;

struct SourceDestinationEntry{
    source: Vec<i32>,
    destination: Vec<i32>,
}

impl SourceDestinationEntry {
    fn new(dest_start: i32, source_start: i32, range: i32) -> SourceDestinationEntry {
        SourceDestinationEntry {
            source : (source_start..source_start + range).collect(),
            destination: (dest_start..dest_start + range).collect(),
        }
    }

    fn to_hashmap(&self) -> HashMap<i32,i32>{
        let ret: HashMap<i32, i32> = self.source.clone().into_iter().zip(self.destination.clone().into_iter()).collect();
        return ret;
    }
}

fn combine_entries(entries: Vec<SourceDestinationEntry>) -> HashMap<i32,i32> {
    return match entries.len() {
        0 => { HashMap::new() }
        _ => {
            let mut ret: HashMap<i32, i32> = HashMap::new();
            for map in entries {
                ret.extend(map.to_hashmap());
            }
            ret
        }
    }
}

struct Map {
    key: String,
    mapping: HashMap<i32, i32>,
    next_map: Option<Box<Map>>, // Optional next map
}

impl Map {

    pub fn new(key: &str, mapping_entries: Vec<SourceDestinationEntry>) -> Map {

        Map {
            key: String::from(key),
            mapping: combine_entries(mapping_entries),
            next_map: None,
        }
    }

    pub fn set_next_map(&mut self, next_map: Map) {
        self.next_map = Some(Box::new(next_map));
    }


    pub fn source_to_destination(&self, source_value: i32) -> i32{
        let destination = self.mapping.get(&source_value);
        return match destination {
            Some(dest) => {
                match &self.next_map {
                    Some(map) => {
                        map.source_to_destination(*dest)
                    }
                    None => { *dest }
                }
            }
            None => {
                source_value
            }
        };
    }
}



fn main() {
    // seed
    let seed_entry_1 = SourceDestinationEntry::new(50,98,2);
    let seed_entry_2 = SourceDestinationEntry::new(52,50,48);

    // soil
    let soil_entry_1 = SourceDestinationEntry::new(0,15,37);
    let soil_entry_2 = SourceDestinationEntry::new(37,52,2);
    let soil_entry_3 = SourceDestinationEntry::new(39,0,15);

    // fertilizer
    let fert_entry_1 = crate::SourceDestinationEntry::new(49,53,8);
    let fert_entry_2 = crate::SourceDestinationEntry::new(0, 11, 42);
    let fert_entry_3 = crate::SourceDestinationEntry::new(42, 0 , 7);
    let fert_entry_4 = crate::SourceDestinationEntry::new(57, 7, 4);

    // water
    let water_entry_1 = crate::SourceDestinationEntry::new(88, 18, 7);
    let water_entry_2 = crate::SourceDestinationEntry::new(18, 25, 70);

    // light
    let light_entry_1 = crate::SourceDestinationEntry::new(45, 77, 23);
    let light_entry_2 = crate::SourceDestinationEntry::new(81, 45, 19);
    let light_entry_3 = crate::SourceDestinationEntry::new(68, 64, 13);

    // temperature
    let temperature_entry_1 = crate::SourceDestinationEntry::new(0, 69, 1);
    let temperature_entry_2 = crate::SourceDestinationEntry::new(1, 0, 69);

    // humidity
    let humidity_entry_1 = crate::SourceDestinationEntry::new(60, 56, 37);
    let humidity_entry_2 = crate::SourceDestinationEntry::new(56, 93, 4);

    let mut seed_map = Map::new("seed", vec![seed_entry_1, seed_entry_2]);
    let mut soil_map = Map::new("soil", vec![soil_entry_1, soil_entry_2, soil_entry_3]);
    let mut fert_map = Map::new("fertilizer", vec![fert_entry_1, fert_entry_2, fert_entry_3, fert_entry_4]);
    let mut water_map = Map::new("water", vec![water_entry_1, water_entry_2]);
    let mut light_map = Map::new("light", vec![light_entry_1, light_entry_2, light_entry_3]);
    let mut temperature_map = Map::new("temperature", vec![temperature_entry_1, temperature_entry_2]);
    let mut humidity_map = Map::new("humidity", vec![humidity_entry_1, humidity_entry_2]);

    // Note: ownership needs the maps to be set bottom up
    temperature_map.set_next_map(humidity_map);
    light_map.set_next_map(temperature_map);
    water_map.set_next_map(light_map);
    fert_map.set_next_map(water_map);
    soil_map.set_next_map(fert_map);
    seed_map.set_next_map(soil_map);

    let seed_1 = seed_map.source_to_destination(79);
    let seed_2 = seed_map.source_to_destination(14);
    let seed_3 = seed_map.source_to_destination(55);
    let seed_4 = seed_map.source_to_destination(13);

    println!("...")
}
