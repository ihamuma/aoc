use std::fs;
use std::num::NonZero;
use grid2d::{Coordinate, Grid};

pub fn solve(file_path: &str) {
    let input_file: String = fs::read_to_string(file_path).unwrap();
    let printing_room_matrix: Vec<Vec<char>> = input_file.lines().map(|l|l.chars().collect()).collect();
    let width: NonZero<usize> = NonZero::new(printing_room_matrix[0].len()).unwrap();
    let flattened_matrix: Vec<char> = printing_room_matrix.into_iter().flatten().collect();
    // Unsafe okay here as the input is of standard width
    let mut printing_room_grid: Grid<char> = unsafe { Grid::new_unchecked(width, flattened_matrix) };

    let (accessible_paper_rolls, inaccessible_paper_rolls): (Vec<grid2d::Coordinate>, Vec<grid2d::Coordinate>) = accessible_paper_rolls(&printing_room_grid);
    println!("Accessible rolls: {}", accessible_paper_rolls.len());

    let removable_rolls = solve_part_two(& mut printing_room_grid, &accessible_paper_rolls, &inaccessible_paper_rolls);
    println!("Removable rolls: {removable_rolls}");
}

fn accessible_paper_rolls(paper_room_grid: &Grid<char>) -> (Vec<grid2d::Coordinate>, Vec<grid2d::Coordinate>) {
    let mut accessible_paper_rolls: Vec<Coordinate>= vec![];
    let mut inaccessible_paper_rolls: Vec<Coordinate>= vec![];
    for (coordinate, item) in paper_room_grid.enumerate() {
        if *item == '@' && paper_room_grid.neighbors(coordinate).map(|i|i.1).filter(|c| **c == '@').count() < 4 {
            accessible_paper_rolls.push(coordinate);
        } else if *item == '@' {
            inaccessible_paper_rolls.push(coordinate);
        }
    }
    (accessible_paper_rolls, inaccessible_paper_rolls)
}

fn solve_part_two(paper_room_grid: & mut Grid<char>, accessible_rolls: &[Coordinate], inaccessible_rolls: &[Coordinate]) -> u32 {
    for coordinate in accessible_rolls {
        println!("{}", paper_room_grid.get(coordinate).unwrap());
        let _ = paper_room_grid.get_mut(coordinate).insert(&mut '.');
        println!("{}", paper_room_grid.get(coordinate).unwrap());
    }
    32
}