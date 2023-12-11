fn read_input() -> Vec<Vec<i32>> {
    let ret: Vec<Vec<i32>> = include_bytes!("../../example.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            line.split(|b| b == &b' ')
                .map(|c| atoi::atoi(c).unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    ret
}

fn build_triangle(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut triangle: Vec<Vec<i32>> = vec![nums.clone()];
    let num_elements = &nums.len();

    for i in 0..*num_elements {
        if triangle[i].iter().all(|val| *val == 0){
            break; // end of triangle
        }

        let next: Vec<i32> = triangle[i]
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        triangle.push(next);
    }
    println!("Breakpoint"); // TODO: Not finished yet, need to expand last list entry
    return triangle;
}

pub fn main() {
    let nums: Vec<Vec<i32>> = read_input();
    let triangle: Vec<Vec<i32>> = build_triangle(nums[0].clone());
    println!("Day 09 Part A!");
}
