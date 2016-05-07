extern crate gol_rust;
use std::collections::HashSet;

fn main() {
    let v = vec![(25,25), (24,25), (24,26), (25, 24), (26, 25)];
    let initial_state: HashSet<_> = v.iter().cloned().collect();
    let next_state_1 = gol_rust::gol_1::next(&v);
    let next_state_2 = gol_rust::gol_2::next(&initial_state);
    for cell in next_state_1 {
        println!("({},{})", cell.0, cell.1)
    }
    println!("-------------------------");
    for cell in next_state_2 {
        println!("({},{})", cell.0, cell.1)
    }
}
