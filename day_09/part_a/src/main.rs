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

    // Build the triangle
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

    // Extent the triangle
    for i in 0..triangle.len() - 1 {
        let last_element_x: i32 = triangle[i].last().cloned().unwrap_or(0);
        let last_element_x1: i32 = triangle[i + 1].last().cloned().unwrap_or(0);

        let sum_last_two: i32 = last_element_x - last_element_x1;

        triangle[i].push(sum_last_two);
        // TODO: Add last elements of 2 continues rows as last element


    }
    // let tri_len = &triangle.len();
    // for (i, row) in triangle.iter_mut().enumerate() {
    //     // Condition for last row, extent the array
    //     if i == tri_len - 1 {
    //         let last_element = row[&row.len() - 1];
    //         row.push(last_element);
    //     }
    //     else {
    //         let l1 = &row.len() - 1;
    //         let l2 = &row.len() - 2;
    //         println!("len: {}", &row.len());
    //         eprintln!("Trying to get {} and {}", &l1, &l2);
    //         let sum_last_two = row[l1] + row[l2];
    //         row.push(sum_last_two);
    //     }
    // }

    println!("Breakpoint"); // TODO: Not finished yet, need to expand last list entry
    return triangle;
}

pub fn main() {
    let nums: Vec<Vec<i32>> = read_input();
    let triangle: Vec<Vec<i32>> = build_triangle(nums[0].clone());

    for array in triangle {
        println!("{:?}", array);
    }
    println!("Day 09 Part A!");
}
