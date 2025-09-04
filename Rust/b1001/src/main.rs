use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí
    let mut a = String::new();
    let mut b = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Error");

    io::stdin()
        .read_line(&mut b)
        .expect("Error");

    let a_int: i32 = a.trim().parse().unwrap();
    let b_int: i32 = b.trim().parse().unwrap();

    let x: i32 = a_int + b_int;
    println!("X = {}", x);
}
