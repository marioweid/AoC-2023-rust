use std::fs;

fn read_data(filepath: &str) -> (Vec<f32>, Vec<f32>) {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    let mut lines = data.split("\n");
    let times: Vec<f32> = lines
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .map(|val| val.parse().unwrap()).collect();

    let distances: Vec<f32> = lines
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .map(|val| val.parse().unwrap()).collect();

    return (times, distances);
}

#[allow(dead_code)]
fn distance_function(button: f32, max_time: f32) -> f32{
    let ret: f32 = -(button*button) + max_time * button;
    return ret
}

fn quadratic_formula(a:f32 , b: f32, c: f32) -> (f32, f32) {
    let disc :f32 = b*b - 4.0*a*c;
    
    let sol_1: f32 = (-b - disc.sqrt()) / (2.0*a);
    let sol_2: f32 = (-b + disc.sqrt()) / (2.0*a);

    return (sol_1.ceil(), sol_2.floor())

}

fn main() {
    let (times,distances)  = read_data("../input.txt");

    let mut amount_solutions: Vec<f32> = Vec::new();
    for (time, dist) in times.iter().zip(distances.iter()){
        let (mut min_sol, mut max_sol) = quadratic_formula(1f32 , -*time, *dist);

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
    let solution_product: f32 = amount_solutions.iter().product();
    println!("Product of all solutions: {}", solution_product);
}
