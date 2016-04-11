extern crate gol_rust;

fn main() {
    let next_state = gol_rust::gol::next(&vec![(0,0)]);
    for cell in next_state {
        println!("({},{})", cell.0, cell.1)
    }
    println!("Hello, world!");
}
