use std::io;

fn main() {
    const PI: f64 = 3.14159;
    let mut R = String::new();

    io::stdin().read_line(&mut R).unwrap();

    let R: f64 = R.trim().parse().unwrap();

    println!("VOLUME = {:.3}", (4.0 / 3.0) * PI * (R * R * R))
}
