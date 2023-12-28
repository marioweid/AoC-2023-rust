use std::fs;

fn read_file(file_path: &str) -> Vec<Vec<Vec<char>>> {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let blocks = content.split("\n\n")
    let blocks: Vec<&str> = content.split("\n\n").collect();
    
    let grids: Vec<Vec<Vec<char>>> = blocks.into_iter().map(|block|{
        return block
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    })
    .collect();
    println!("{:?}",grids);

    grids
}

fn find_mirror(grid: Vec<Vec<char>>) -> i32{
    for r in 1..grid.len(){
        let mut above: Vec<Vec<char>> = grid.iter().take(r).cloned().rev().collect();
        let mut below: Vec<Vec<char>> = grid.iter().skip(r).cloned().collect();

        // let min_len = above.len().min(below.len());

        // above.truncate(min_len);
        // below.truncate(min_len);

        
        let diffs = above
        .iter()
        .zip(below)
        .map(|(x, y)| {
            x.iter().zip(y.iter()).filter(|(a, b)| a != b).count()
        })
        .sum::<usize>();


        if diffs == 1 {
            return r as i32;
        }
    }

    return 0
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_cols = grid.first().unwrap().len();
    let mut row_iters: Vec<_> = grid.into_iter().map(Vec::into_iter).collect();
    let ret = (0..num_cols)
        .map(|_| row_iters.iter_mut().map(|it| it.next().unwrap()).collect())
        .collect();
    return ret;
}

pub fn main() {
    let mut total :i32 = 0;
    let grids = read_file("../input.txt");

    for grid in grids{
        let row_index: i32 = find_mirror(grid.clone());
        total += row_index * 100;

        let transposed = transpose(grid);

        let col_index: i32 = find_mirror(transposed);
        total += col_index ;
    }
    println!("Result: {}", total);
}
