use std::fs;

fn read_data(filepath: &str) -> (Vec<f64>, Vec<f64>) {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    let mut lines = data.split("\n");
    let times: Vec<f64> = lines
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .map(|val| val.parse().unwrap()).collect();

    let time: f64 = times.into_iter().map(|num| num.to_string()).collect::<String>().parse().unwrap();
    
    let distances: Vec<f64> = lines
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .map(|val| val.parse().unwrap()).collect();

    let distance: f64 = distances.into_iter().map(|num| num.to_string()).collect::<String>().parse().unwrap();

    return (vec![time], vec![distance]);
}

#[allow(dead_code)]
fn distance_function(button: f64, max_time: f64) -> f64{
    let ret: f64 = -(button*button) + max_time * button;
    return ret
}

fn quadratic_formula(a:f64 , b: f64, c: f64) -> (f64, f64) {
    let disc :f64 = b*b - 4.0*a*c;
    
    let sol_1: f64 = (-b - disc.sqrt()) / (2.0*a);
    let sol_2: f64 = (-b + disc.sqrt()) / (2.0*a);

    return (sol_1.ceil(), sol_2.floor())

}

fn main() {
    let (times,distances)  = read_data("../input.txt");

    let mut amount_solutions: Vec<f64> = Vec::new();
    for (time, dist) in times.iter().zip(distances.iter()){
        let (mut min_sol, mut max_sol) = quadratic_formula(1f64 , -*time, *dist);

        // Check edge cases because its the distance needs to be over the current and not the same
        let min_sol_dist = distance_function(min_sol, *time);
        let max_sol_dist = distance_function(max_sol, *time);
        if min_sol_dist == *dist{
            min_sol = min_sol + 1.0;
        }
        if max_sol_dist == *dist{
            max_sol = max_sol - 1.0;
        }
        let possible_solutions = max_sol-min_sol + 1.0;
        println!("Amount of possible solutions: {}", &possible_solutions);
        amount_solutions.push(possible_solutions)
    }
}
