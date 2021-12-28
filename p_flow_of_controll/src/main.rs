fn main() {
    println!("Hello, world!");
    p_if_else();
}

fn p_if_else () {
    let n = -5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}