fn main() {
    println!("Hello, world!");

    used_function();
}

fn used_function () {}

#[allow(dead_code)]
fn unused_function () {}