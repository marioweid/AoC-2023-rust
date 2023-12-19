use std::{
    collections::{VecDeque, HashSet},
    fs,
};

fn read_file(file_path: &str) -> Vec<Vec<char>> {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    return grid;
}

fn find_start_position(grid: &Vec<Vec<char>>) -> Result<(usize, usize), &'static str> {
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == 'S' {
                return Ok((r, c));
            }
        }
    }
    Err("Cant find Starting-Position inside the grid.")
}

// fn fluid_fill(grid: &mut Vec<Vec<&str>>, row: usize, col: usize, new_value: &str, old_value: &str) {
//     // Check if the current cell is within the grid and has the old value
//     if row < grid.len() && col < grid[0].len() && grid[row][col] == old_value {
//         // Fill the current cell with the new value
//         grid[row][col] = new_value;

//         // Recursively fill the neighboring cells
//         fluid_fill(grid, row + 1, col, new_value, old_value); // Down
//         fluid_fill(grid, row, col + 1, new_value, old_value); // Right
//         if row > 0 {
//             fluid_fill(grid, row - 1, col, new_value, old_value); // Up
//         }
//         if col > 0 {
//             fluid_fill(grid, row, col - 1, new_value, old_value); // Left
//         }
//     }
// }

fn fill(
    grid: &Vec<Vec<char>>,
    mut queue: VecDeque<(usize, usize)>,
    mut vals_loop: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    while !queue.is_empty() {
        let (row_idx, col_idx) = queue.pop_back().unwrap();
        let current_val = grid[row_idx][col_idx];

        if row_idx > 0
            && "S|JL".contains(current_val)
            && "|7F".contains(grid[row_idx - 1][col_idx])
            && !vals_loop.contains(&(row_idx - 1, col_idx))
        {
            vals_loop.insert((row_idx - 1, col_idx));
            queue.push_front((row_idx - 1, col_idx));
        }

        if row_idx < grid.len() - 1 
            && "S|7F".contains(current_val)
            &&  "|JL".contains(grid[row_idx + 1][col_idx]) 
            && !vals_loop.contains(&(row_idx +1, col_idx)){
                vals_loop.insert((row_idx + 1, col_idx));
                queue.push_front((row_idx + 1, col_idx));
        }

        if col_idx > 0 
        && "S-J7".contains(current_val)
        && "-LF".contains(grid[row_idx][col_idx -1])
        && !vals_loop.contains(&(row_idx, col_idx - 1)) {
            vals_loop.insert((row_idx, col_idx - 1));
            queue.push_front((row_idx, col_idx - 1));
        }

        if col_idx < grid[row_idx].len() - 1 
        && "S-LF".contains(current_val) 
        && "-J7".contains(grid[row_idx][col_idx + 1])
        && !vals_loop.contains(&(row_idx, col_idx + 1)) {
            vals_loop.insert((row_idx, col_idx +1));
            queue.push_front((row_idx, col_idx +1));
        }
    }
    return vals_loop;

    // if c < len(grid[r]) - 1 and ch in "S-LF" and grid[r][c + 1] in "-J7" and (r, c + 1) not in loop:
    //     loop.add((r, c + 1))
    //     q.append((r, c + 1))
}

pub fn main() {
    let grid: Vec<Vec<char>> = read_file("../input.txt");
    let (start_row, start_col) = find_start_position(&grid).unwrap();
    println!("S: ({},{})", &start_row, &start_col);

    let mut vals_loop: HashSet<(usize, usize)> = HashSet::new();
    vals_loop.insert((start_row, start_col));

    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));
    let result = fill(&grid, queue, vals_loop);
    
    println!("loop_vals: {}", result.len() / 2);
}
