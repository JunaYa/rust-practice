fn main() {
    println!("Hello, world!");

    matchFn1();
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