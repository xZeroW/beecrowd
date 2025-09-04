use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí
    const PI: f64 = 3.14159;
    let mut raio = String::new();

    io::stdin().read_line(&mut raio).expect("Oops...");

    let mut f_raio: f64 = raio.trim().parse().unwrap();
    f_raio = f_raio * f_raio;
    
    println!("A={:.4}", f_raio * PI)
}