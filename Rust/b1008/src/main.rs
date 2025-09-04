use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let number: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let hours: f64 = lines.next().unwrap().unwrap().parse().unwrap();
    let sal: f64 = lines.next().unwrap().unwrap().parse().unwrap();

    println!("NUMBER = {}", number);
    println!("SALARY = U$ {:.2}", sal * hours);
}
