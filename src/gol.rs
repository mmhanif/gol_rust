//! Simple functional implementation of Conway' Game of Life (GoL) using Vectors
//!
//! Basic GoL implementation using Vectors of 2-tuples to model state

/// Given a vector of 2-tuples representing initial state of universe
/// returns a new vector giving the next state of the universe
pub fn next(state: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut next_state = vec![];
    for cell in neighborhood(&state) {
        match num_living_neighbors(&cell, &state) {
            3                           => next_state.push((cell.0, cell.1)),
            2 if is_alive(&cell, state) => next_state.push((cell.0, cell.1)),
            _                           => {},
        }
    }
    next_state
}

// Given a specific cell and state of the universe, return the number of living
// neighbors of that cell
fn num_living_neighbors(cell: &(i32, i32), state: &Vec<(i32,i32)>) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            if !(i == 0 && j == 0) {
                if is_alive(&(cell.0 + i, cell.1 + j), state) {
                    count += 1;
                    //println!("({},{}) found neighbor at ({},{})", cell.0, cell.1, cell.0 + i, cell.1 + j);
                }
            }
        }
    }
    //println!("({},{}) neighbors = {}", cell.0, cell.1, count);
    count
}

// Return a Vector of living cells and all their neighboring cells
fn neighborhood(state: &Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    let mut the_hood = vec![];
    for cell in state {
        if !the_hood.contains(cell) {
            the_hood.push((cell.0, cell.1));
            //println!("-- Added ({},{}) to neighborhood", cell.0, cell.1);
        }
        for neighbor in neighbors(&cell) {
            if !the_hood.contains(&neighbor) {
                the_hood.push((neighbor.0, neighbor.1));
                //println!("Added ({},{}) to neighborhood", neighbor.0, neighbor.1);
            }
        }
    }

    the_hood
}

// Basic helper, returns true if given cell is present in given state of the universe
fn is_alive(cell: &(i32,i32), state: &Vec<(i32,i32)>) -> bool {
    state.contains(&cell)
}

// For a given cell, represented as a 2-tuple, return a vector of all neighboring cells
fn neighbors(cell: &(i32,i32)) -> Vec<(i32,i32)> {
    let mut the_neighbors = vec![];
    for i in -1..2 {
        for j in -1..2 {
            if !(i == 0 && j == 0) {
                    the_neighbors.push((cell.0 + i, cell.1 + j));
                }
            }
        }
    the_neighbors
}

// Tests **********

#[test]
fn empty_universe_stays_empty() {
    let initial_state = vec![];
    let next_state = next(&initial_state);
    assert_eq!(initial_state, next_state);
}

#[test]
fn living_cell_with_no_living_neighbors_dies() {
    let initial_state = vec![(0,0)];
    let next_state: Vec<(i32,i32)> = next(&initial_state);
    let expected_state: Vec<(i32,i32)> = vec![];
    assert_eq!(expected_state, next_state);
}

#[test]
fn living_cell_with_two_living_neighbors_lives() {
    // *  *       *  *
    // *      =>  * [*]
    let initial_state = vec![(0,0), (1,0), (0,1)];
    let next_state: Vec<(i32,i32)> = next(&initial_state);
    assert!(next_state.contains(&(0,0)));
    assert!(next_state.contains(&(1,0)));
    assert!(next_state.contains(&(0,1)));
}

#[test]
fn living_cell_with_four_living_neighbors_lives() {
    let initial_state = vec![(0,0), (-1,-1), (-1,1), (1,-1), (1, 1)];
    let next_state: Vec<(i32,i32)> = next(&initial_state);
    assert!(!next_state.contains(&(0,0)));
}

#[test]
fn dead_cell_with_three_living_neighbors_lives() {
    // *  *       [*] [*]
    // *      =>  [*]  *
    let initial_state = vec![(0,0), (1,0), (0,1)];
    let next_state: Vec<(i32,i32)> = next(&initial_state);
    assert!(next_state.contains(&(1,1)));
}

#[test]
fn full_pattern_test() {
    // *  .  *      .  *  .
    // .  *  .  =>  *  .  *
    // *  .  *      .  *  .
    let initial_state = vec![(0,0), (-1,-1), (-1,1), (1,-1), (1, 1)];
    let mut next_state: Vec<(i32,i32)> = next(&initial_state);
    let mut expected_state: Vec<(i32,i32)> = vec![(-1,0), (0,-1), (1,0), (0,1)];
    assert_eq!(expected_state.sort(), next_state.sort());
}
