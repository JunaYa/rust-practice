fn main() {
    println!("Hello, world!");

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num1: Number = int.into();
    println!("My number2 is {:?}", num1);
}

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}