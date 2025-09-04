use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí

    let mut str_nota_a = String::new();
    let mut str_nota_b = String::new();
    let mut str_nota_c = String::new();

    let peso_a: f64 = 2.0;
    let peso_b: f64 = 3.0;
    let peso_c: f64 = 5.0;

    io::stdin().read_line(&mut str_nota_a).unwrap();
    io::stdin().read_line(&mut str_nota_b).unwrap();
    io::stdin().read_line(&mut str_nota_c).unwrap();

    let nota_a: f64 = str_nota_a.trim().parse().expect("msg");
    let nota_b: f64 = str_nota_b.trim().parse().expect("msg");
    let nota_c: f64 = str_nota_c.trim().parse().expect("msg");

    let soma_produtos = (nota_a * peso_a) + (nota_b * peso_b) + (nota_c * peso_c);
    let soma_pesos = peso_a + peso_b + peso_c;
    let media = soma_produtos / soma_pesos;

    println!("MEDIA = {:.1}", media)
}
