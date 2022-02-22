fn main() {
    println!("Hello, world!");

    let (x, y) = (2, 3);
    print_refs(&x, &y);

    println!("out x is {}", &x);

    borrow_error();
}

fn print_refs<'a, 'b> (x: &'a i32, y: &'b i32) {
    println!("x is {} y is {}", x, y);
}

fn borrow_error<'a> () {
    let _x = 2;
    // let y: &'a i32 = &_x;
}