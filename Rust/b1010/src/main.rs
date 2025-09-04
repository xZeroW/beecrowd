use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let pc1 = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = pc1.split_whitespace().collect();
    let pc1qnt: f64 = parts[1].parse().unwrap();
    let pc1price: f64 = parts[2].parse().unwrap();

    let pc2 = lines.next().unwrap().unwrap();
    let parts2: Vec<&str> = pc2.split_whitespace().collect();
    let pc2qnt: f64 = parts2[1].parse().unwrap();
    let pc2price: f64 = parts2[2].parse().unwrap();

    println!("VALOR A PAGAR: R$ {:.2}", (pc1qnt * pc1price) + (pc2qnt * pc2price));
}
