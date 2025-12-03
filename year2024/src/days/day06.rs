use std::collections::HashSet;
use std::fs;

pub fn solve(input_file: &str) {
    let mut matrix: Vec<Vec<char>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|x: &str| x.chars().collect())
        .collect();

    let len_y: i16 = i16::try_from(matrix.len()).unwrap();
    let len_x: i16 = i16::try_from(matrix.len()).unwrap();

    let Some(starting_position) = find_char(&matrix, '^') else { panic!("Starting position not found in matrix!") };

    // Mark starting position as already visited, avoid double counting
    matrix[starting_position.0][starting_position.1] = 'X';

    // Cast as i16 to allow checking for negative values
    let mut i: i16 = i16::try_from(starting_position.0).unwrap();
    let mut j: i16 = i16::try_from(starting_position.0).unwrap();
    let mut dir: &str = "up";
    let mut positions: u16 = 1;

    loop {
        let new_pos: (i16, i16) = move_in_dir(dir, i, j);
        i = new_pos.0;
        j = new_pos.1;

        if is_outside((i, j), len_y, len_x) {
            break;
        }

        // If !is_outside, both i & j are positive and can be cast to usize to get next char
        #[allow(clippy::cast_sign_loss)]
        let y: usize = i as usize;
        #[allow(clippy::cast_sign_loss)]
        let x: usize = j as usize;
        let next: char = matrix[y][x];

        // If next is an obstacle, go back to previous and turn right
        if next == '#' {
            let prev_pos: (i16, i16) = move_back(dir, i, j);
            i = prev_pos.0;
            j = prev_pos.1;
            dir = turn_right(dir);
        // If next has been previously visited, ignore
        } else if next == 'X' {
        // Else, mark as visited, add one to positions count and continue to next position.
        } else {
            matrix[y][x] = 'X';
            positions += 1;
        }
    }

    println!("The guard visits {positions} distinct positions before leaving the area");

    // Reset: mark starting as P to signal previously checked obstacle position
    matrix[starting_position.0][starting_position.1] = 'P';
    i = i16::try_from(starting_position.0).unwrap();
    j = i16::try_from(starting_position.0).unwrap();
    dir = "up";

    // Create separate iterators for obstacle placement
    let mut obs_i: i16 = i;
    let mut obs_j: i16 = j;
    let mut obs_dir = dir;
    let mut looping_obstacles: u16 = 0;

    'obstacle_placing: loop {
        // Place obstacle according to previous position and direction
        let new_obs_pos: (i16, i16) = move_in_dir(obs_dir, obs_i, obs_j);
        obs_i = new_obs_pos.0;
        obs_j = new_obs_pos.1;

        if is_outside((obs_i, obs_j), len_y, len_x) {
            break 'obstacle_placing;
        }

        #[allow(clippy::cast_sign_loss)]
        let obs_y: usize = obs_i as usize;
        #[allow(clippy::cast_sign_loss)]
        let obs_x: usize = obs_j as usize;
        let next_obs: char = matrix[obs_y][obs_x];

        // If next obstacle placement is an original obstacle, go back to previous and turn right
        if next_obs == '#' {
            let prev_obs_pos: (i16, i16) = move_back(obs_dir, obs_i, obs_j);
            obs_i = prev_obs_pos.0;
            obs_j = prev_obs_pos.1;
            obs_dir = turn_right(obs_dir);
            continue;
        // If next has previously been checked for looping, continue
        } else if next_obs == 'P' {
            continue;
        // Else place new obstacle
        } matrix[obs_y][obs_x] = 'O';

        // Place "test guard" in position before new obstacle
        let new_start_pos: (i16, i16) = move_back(obs_dir, obs_i, obs_j);
        i = new_start_pos.0;
        j = new_start_pos.1;

        // Initialise HashSet to store previous locations
        let mut location_map: HashSet<(&str, (i16, i16))> = HashSet::new();

        // Turn right to not hit new obstacle immediately
        dir = turn_right(obs_dir);

        'loop_or_exit: loop {
            // Move to next position
            let new_pos: (i16, i16) = move_in_dir(dir, i, j);
            i = new_pos.0;
            j = new_pos.1;

            // If guard makes it outside, mark as previously obstacled and break loop
            if is_outside((i, j), len_y, len_x) {
                matrix[obs_y][obs_x] = 'P';
                break 'loop_or_exit;
            }

            // If same dir, location pair already in location_map
            // -> second time to that location with same dir
            // -> guard looped, add to looped count, break loop.
            if location_map.contains(&(dir, (i, j))) {
                looping_obstacles += 1;
                matrix[obs_y][obs_x] = 'P';
                break 'loop_or_exit;
            } location_map.insert((dir, (i, j)));

            #[allow(clippy::cast_sign_loss)]
            let y: usize = i as usize;
            #[allow(clippy::cast_sign_loss)]
            let x: usize = j as usize;
            let next: char = matrix[y][x];

            if next == '#' || next == 'O' {
                let prev_pos: (i16, i16) = move_back(dir, i, j);
                i = prev_pos.0;
                j = prev_pos.1;
                dir = turn_right(dir);
            }
        }
    }

    println!("There are {looping_obstacles} ways to place an object and create an infinite loop");
}

fn find_char(matrix: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == target) {
            return Some((i, j));
        }
    }
    None
}

fn move_in_dir(dir: &str, i: i16, j: i16) -> (i16, i16) {
    if dir == "up" {
        return (i - 1, j);
    } else if dir == "down" {
        return (i + 1, j);
    } else if dir == "left" {
        return (i, j - 1);
    } (i, j + 1)
}

fn move_back(dir: &str, i: i16, j: i16) -> (i16, i16) {
    if dir == "up" {
        return (i + 1, j);
    } else if dir == "down" {
        return (i - 1, j);
    } else if dir == "left" {
        return (i, j + 1);
    } (i, j - 1)
}

const fn is_outside(pos: (i16, i16), len_y: i16, len_x: i16) -> bool {
    let y = pos.0;
    let x = pos.1;
    if y < 0 || len_y <= y || x < 0 || len_x <= x {
        return true;
    }
    false
}

fn turn_right(dir: &str) -> &'static str {
    if dir == "up" {
        return "right";
    } else if dir == "right" {
        return "down";
    } else if dir == "down" {
        return "left";
    } "up"
}
