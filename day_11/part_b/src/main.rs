use std::fs;

fn read_file(file_path: &str) -> Vec<Vec<char>> {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    return grid;
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
    let grid: Vec<Vec<char>> = read_file("../input.txt");

    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&char| char == '.'))
        .map(|(r, _)| r)
        .collect();

    let empty_cols: Vec<usize> = transpose(grid.clone())
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&char| char == '.'))
        .map(|(c, _)| c)
        .collect();

    let points: Vec<(usize, usize)> = grid
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &value)| value == '#')
                .map(move |(c, _)| (r, c))
        })
        .collect();

    let scale: i64 = 1000000;
    let mut total:i64 = 0;

    points.iter().enumerate().for_each(|(i,(r1, c1))|{
        points.iter().cloned().take(i).for_each(|(r2, c2)| {
            for r in *r1.min(&r2)..*r1.max(&r2){
                total += if empty_rows.contains(&r) {scale} else {1}; 
            }
            for c in *c1.min(&c2)..*c1.max(&c2){
                total += if empty_cols.contains(&c) {scale} else {1}; 
            }
        })
    });

    println!("Total: {}", total);


}

// 0. read inpout into grid
// 1. find empty rows
// 2. find all empty columns
// 3. get the points (x, y)
// 4. Compare min(point_a) with every other max(point_b) in grid
//   1. Calculate total if Empty row
//      1. if row empty => add 1000000
//      2. If not       => add 1
//  2. Calculate total if Empty column
//      1. if col empty => add 1000000
//      2. If not       => add 1

