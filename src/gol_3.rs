//! Optimized implementation of Conway' Game of Life (GoL) that attempts
//! to ony iterate over living cells once
//!
//! Basic GoL implementation using Vectors of 2-tuples to model state
use std::collections::HashSet;
use std::collections::HashMap;

/// Given a vector of 2-tuples representing initial state of universe
/// returns a new vector giving the next state of the universe
pub fn next(state: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut next_state = HashSet::new();
    let mut dead_neighbors = HashMap::new();

    // Iterate over living cells, get neighbors of each cell,
    // if neighbor is living, increment living_neighbor_count for cell
    // if neighbor is dead, increment counter in HashMap of dead neighbor cells
    // and their living neighbor count
    for cell in state {
        let mut living_neighbor_count = 0;
        for neighbor in neighbors(&cell) {
            if is_alive(&neighbor, state) {
                living_neighbor_count += 1;
            }
            else {
                let count = dead_neighbors.entry(neighbor).or_insert(0);
                *count += 1;
            }
        }
        let _ = match living_neighbor_count {
            3 | 2 => next_state.insert((cell.0, cell.1)),
            _     => false,
        };
    }

    // Now iterate over dead neighbors and check the living neighbor count of each
    for (dead_cell, living_neighbor_count) in dead_neighbors {
        //println!("({},{}) neighbors = {}", dead_cell.0, dead_cell.1, living_neighbor_count);
        if living_neighbor_count == 3 {
            next_state.insert((dead_cell.0, dead_cell.1));
        }
    }
    next_state
}

// Basic helper, returns true if given cell is present in given state of the universe
fn is_alive(cell: &(i32,i32), state: &HashSet<(i32,i32)>) -> bool {
    state.contains(&cell)
}

// For a given cell, represented as a 2-tuple, return a vector of all neighboring cells
fn neighbors(cell: &(i32,i32)) -> HashSet<(i32,i32)> {
    let mut the_neighbors = HashSet::new();
    for i in -1..2 {
        for j in -1..2 {
            if !(i == 0 && j == 0) {
                    the_neighbors.insert((cell.0 + i, cell.1 + j));
                }
            }
        }
    the_neighbors
}

// Tests **********

#[test]
fn empty_universe_stays_empty() {
    let initial_state: HashSet<(i32,i32)> = HashSet::new();
    let next_state = next(&initial_state);
    assert_eq!(initial_state, next_state);
}

#[test]
fn living_cell_with_no_living_neighbors_dies() {
    let initial_state: HashSet<_> = vec![(0,0)].iter().cloned().collect();
    let next_state: HashSet<(i32,i32)> = next(&initial_state);
    let expected_state: HashSet<(i32,i32)> = HashSet::new();
    assert_eq!(expected_state, next_state);
}

#[test]
fn living_cell_with_two_living_neighbors_lives() {
    // *  *       *  *
    // *      =>  * [*]
    let initial_state: HashSet<_> = vec![(0,0), (1,0), (0,1)].iter().cloned().collect();
    let next_state: HashSet<(i32,i32)> = next(&initial_state);
    assert!(next_state.contains(&(0,0)));
    assert!(next_state.contains(&(1,0)));
    assert!(next_state.contains(&(0,1)));
}

#[test]
fn living_cell_with_four_living_neighbors_lives() {
    let initial_state: HashSet<_> = vec![(0,0), (-1,-1), (-1,1), (1,-1), (1, 1)].iter().cloned().collect();
    let next_state: HashSet<(i32,i32)> = next(&initial_state);
    assert!(!next_state.contains(&(0,0)));
}

#[test]
fn dead_cell_with_three_living_neighbors_lives() {
    // *  *       [*] [*]
    // *      =>  [*]  *
    let initial_state: HashSet<_> = vec![(0,0), (1,0), (0,1)].iter().cloned().collect();
    let next_state: HashSet<(i32,i32)> = next(&initial_state);
    assert!(next_state.contains(&(1,1)));
}

#[test]
fn full_pattern_test() {
    // *  .  *      .  *  .
    // .  *  .  =>  *  .  *
    // *  .  *      .  *  .
    let initial_state: HashSet<_> = vec![(0,0), (-1,-1), (-1,1), (1,-1), (1, 1)].iter().cloned().collect();
    let next_state: HashSet<(i32,i32)> = next(&initial_state);
    let expected_state: HashSet<(i32,i32)> = vec![(-1,0), (0,-1), (1,0), (0,1)].iter().cloned().collect();
    assert_eq!(expected_state, next_state);
}
