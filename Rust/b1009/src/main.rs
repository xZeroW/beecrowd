use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _name = lines.next().unwrap().unwrap();
    let sal: f64 = lines.next().unwrap().unwrap().parse().unwrap();
    let sells: f64 = lines.next().unwrap().unwrap().parse().unwrap();

    let total = sal + sells * 0.15;

    println!("TOTAL = R$ {:.2}", total);
}
