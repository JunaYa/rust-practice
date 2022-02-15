#![crate_type = "lib"]
pub fn public_function () {
	println!("called rary`s public_function");
}

fn private_function () {
	println!("called rary`s private_function");
}

pub fn indirect_function () {
	println!("called rary`s indirect_function");
	private_function();
}