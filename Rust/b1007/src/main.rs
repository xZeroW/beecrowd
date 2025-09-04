use std::io::{self};

fn main() {
    let mut lines = io::stdin().lines();

    let a: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let b: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let c: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let d: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    println!("DIFERENCA = {}", a * b - c * d );
}
