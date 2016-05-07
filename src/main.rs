extern crate gol_rust;
use std::collections::HashSet;

#[macro_use]
extern crate timeit;

fn main() {
    let mut v = vec![(25,25), (24,25), (24,26), (25, 24), (26, 25)];
    let mut state: HashSet<_> = v.iter().cloned().collect();

    timeit! ({
        for _ in 0..1000 {
            v = gol_rust::gol_1::next(&v);
        }
    });

    timeit! ({
        for _ in 0..1000 {
            state = gol_rust::gol_2::next(&state);
        }
    });

    //for cell in v {
    //    println!("({},{})", cell.0, cell.1)
    //}
    //println!("-------------------------");
    //for cell in state {
    //    println!("({},{})", cell.0, cell.1)
    //}
}
