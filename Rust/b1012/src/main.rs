use std::io;

fn main() {
    const PI: f64 = 3.14159;
    let mut valores = String::new();

    let _ = io::stdin().read_line(&mut valores);

    let parts: Vec<&str> = valores.split_whitespace().collect();
    let A: f64 = parts[0].parse().unwrap();
    let B: f64 = parts[1].parse().unwrap();
    let C: f64 = parts[2].parse().unwrap();

    println!("TRIANGULO: {:.3}", (A * C) / 2.0);
    println!("CIRCULO: {:.3}", PI * C.powi(2));
    println!("TRAPEZIO: {:.3}", ((A + B) * C) / 2.0);
    println!("QUADRADO: {:.3}", B.powi(2));
    println!("RETANGULO: {:.3}", A * B);
}

