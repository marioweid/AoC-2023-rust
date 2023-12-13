fn read_input() -> Vec<Vec<i32>> {
    let ret: Vec<Vec<i32>> = include_bytes!("../../input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            line.split(|b| b == &b' ')
                .map(|c| atoi::atoi(c).unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    ret
}

fn extrapolate(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|val| *val == 0){
        return 0
    }
    let diffs: Vec<i32> = nums
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();

    // let diffs: Vec<i32> = nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
    return nums.iter().nth(0).unwrap() - extrapolate(diffs)
}

pub fn main() {
    let nums: Vec<Vec<i32>> = read_input();
    let mut line_results: Vec<i32> = Vec::new();
    for line in nums {
        let line_res: i32 = extrapolate(line);
        line_results.push(line_res)
    }
    let result: i32  = line_results.iter().sum();
    println!("Result: {}",result);
}
