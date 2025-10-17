fn main() {
	let toshiba: f32 = 450_000.0;
	let mac: f32 = 1_500_000.0;
	let hp: f32 = 750_000.0;
	let dell: f32 = 2_850_000.0;
	let acer: f32 = 250_000.0;

	// Sum
	let S = toshiba + mac + hp + dell + acer;
	println!("Sum is {}", S);

	// Average 
	let a = S / 5.0;
	println!("Average is {}", a);
}