use std::fs;

fn read_file(file_path: &str) -> (Vec<String>, Vec<Vec<i32>>) {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = content.lines().collect();
    let mut springs: Vec<String> = Vec::new();
    let mut numbers: Vec<Vec<i32>> = Vec::new();

    lines.iter().for_each(|line| {
        let mut line_iter = line.split_ascii_whitespace();
        let spring_config = line_iter.next().map(|s| s.to_string()).unwrap();
        let number_config: Vec<i32> = line_iter
            .next()
            .unwrap()
            .split(",")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        springs.push(spring_config);
        numbers.push(number_config)
    });

    return (springs, numbers);
}

fn combinations(springs: String, numbers: &Vec<i32>) -> i32{
    if springs == ""{
        return if numbers.is_empty() {1} else {0};
    }

    if numbers.is_empty(){
        return if springs.contains("#")  {0} else {1};
    }

    let mut ret: i32 = 0;

    let first_el: char = springs.chars().nth(0).unwrap();    
    if ".?".contains(first_el){
        ret += combinations(springs.chars().skip(1).collect(), numbers);
    }

    if "#?".contains(first_el){
        let n_0: usize = numbers[0] as usize;
        if n_0 <= springs.len() && !(springs.chars().take(n_0).collect::<String>().contains('.')) && (springs.len() == n_0 || springs.chars().nth(n_0).unwrap() != '#'){
            ret += combinations(
                springs.chars().skip(n_0+1).collect(), 
                &numbers.iter().skip(1).cloned().collect());
        }
    }
    ret
}

pub fn main() {
    let (springs, numbers) = read_file("../input.txt");
    let mut results: Vec<i32> = Vec::new();
    for i in 0..springs.len(){
        let res = combinations(springs[i].to_string(), &numbers[i]);
        results.push(res)
    }
    let sum:  i32 = results.iter().sum();
    println!("Result: {}", sum);
}
