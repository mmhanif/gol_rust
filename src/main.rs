extern crate gol_rust;
use std::collections::HashSet;

#[macro_use]
extern crate timeit;

fn main() {
    let mut state_1 = vec![(25,25), (24,25), (24,26), (25, 24), (26, 25)];
    let mut state_2: HashSet<_> = state_1.iter().cloned().collect();
    let mut state_3: HashSet<_> = state_1.iter().cloned().collect();

    timeit! ({
        for _ in 0..1000 {
            state_1 = gol_rust::gol_1::next(&state_1);
        }
    });

    timeit! ({
        for _ in 0..1000 {
            state_2 = gol_rust::gol_2::next(&state_2);
        }
    });

    timeit! ({
        for _ in 0..1000 {
            state_3 = gol_rust::gol_3::next(&state_3);
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
