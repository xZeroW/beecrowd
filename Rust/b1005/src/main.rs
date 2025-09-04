use std::io;

fn main() {
	// Escreva a sua solução aqui
	// Code your solution here
	// Escriba su solución aquí

	let mut input_a: String = String::new();
	let peso_a: f64 = 3.5;
	let mut input_b: String = String::new();
	let peso_b: f64 = 7.5;

	io::stdin()
		.read_line(&mut input_a)
		.unwrap();
	io::stdin()
		.read_line(&mut input_b)
		.unwrap();

	let nota_a: f64 = input_a.trim().parse().unwrap();
	let nota_b: f64 = input_b.trim().parse().unwrap();

	let soma: f64 = (nota_a * peso_a) + (nota_b * peso_b);
	let soma_pesos: f64 = peso_a + peso_b;
	let media: f64 = soma / soma_pesos;

	println!("MEDIA = {:.5}", media)
}
