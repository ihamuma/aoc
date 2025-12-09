use std::fs;
use std::num::NonZero;
use grid2d::Grid;

pub fn solve(file_path: &str) {
    let input_file: String = fs::read_to_string(file_path).unwrap();
    let printing_room_matrix: Vec<Vec<char>> = input_file.lines().map(|l|l.chars().collect()).collect();
    let width: NonZero<usize> = NonZero::new(printing_room_matrix[0].len()).unwrap();
    let flattened_matrix: Vec<char> = printing_room_matrix.into_iter().flatten().collect();
    // Unsafe okay here as the input is of standard width
    let printing_room_grid: Grid<char> = unsafe { Grid::new_unchecked(width, flattened_matrix) };

    let accessible_paper_rolls: u32 = solve_part_one(&printing_room_grid);
    println!("{accessible_paper_rolls}");
}

fn solve_part_one(grid: &Grid<char>) -> u32 {
    let mut accessible_paper_rolls: u32 = 0;
    for (coordinate, _item) in grid.enumerate() {
        if grid.neighbors(coordinate).map(|i|i.1).filter(|c| **c == '@').count() < 4 {
            accessible_paper_rolls += 1;
        }
    }
    accessible_paper_rolls
}