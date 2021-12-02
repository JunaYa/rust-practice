fn main() {
    println!("Hello, world!");

    matchFn1();

    matchFn2();
}

// 匹配字面值
fn matchFn1 () {
    let x = 5;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("None"),
    }
}

// 匹配命名变量
fn matchFn2 () {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y={:?}", y),
        _ => println!("none match"),
    }
}