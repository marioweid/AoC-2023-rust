use std::{collections::HashMap, fs};

fn read_input(filename: &str) -> (String, HashMap<String, (String, String)>) {
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let mut blocks = data.split("\n\n");
    
    let mut transtition_list : Vec<(String, (String, String))> = Vec::new();
    let walk_sequence: String = blocks.next().unwrap().to_string();
    let lines = blocks.next().unwrap().split("\n");

    for line in lines {
        let mut markov_table_split = line.split(" = ");
        let markov_key: &str = markov_table_split.next().unwrap();
        let transitions: Vec<&str> = markov_table_split
        .next().unwrap()
        .trim_matches(|c|  c == '(' || c == ')')
        .split(", ")
        .map(|s| s.trim()).collect();
        let markov_transition: (String, String) = (transitions[0].to_string(), transitions[1].to_string());
    
        transtition_list.push((markov_key.to_string(), markov_transition));
    }
    
    
    
    let transition_map: HashMap<String, (String, String)> = transtition_list.into_iter().collect();
    return (walk_sequence, transition_map)
}

fn estimate_steps_to_target(
    transitions: &HashMap<String, (String, String)>,
    start_state: &str,
    target_state: &str,
    sequence: String
) -> usize {
    let mut current_state = start_state;
    let mut steps_count = 0;
    let rep_seq = sequence.repeat(100);
    let step_order: &[u8] = rep_seq.as_bytes();

        while  current_state != target_state {
            println!("{}",current_state);
            let (next_state_1, next_state_2) = transitions.get(current_state).unwrap();
            current_state = match step_order[steps_count] {
                b'L' => { println!("left"); next_state_1},
                b'R' => {println!("right"); next_state_2},
                _ => {next_state_2}
            };
            steps_count += 1;
            if steps_count == step_order.len() {
                println!("Not terminating in state ({}) after {} steps.",current_state, steps_count);
                break;
            }
        }
    steps_count
}

fn main() {

    let (sequence, markov_chain) = read_input("../input.txt");
    let start_state = "AAA";
    let target_state = "ZZZ";

    let num_steps = estimate_steps_to_target(&markov_chain, start_state, target_state, sequence);
    println!("Expected number of steps to reach ZZZ: {}", num_steps);
}