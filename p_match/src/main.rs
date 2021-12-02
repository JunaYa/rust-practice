fn main() {
    println!("Hello, world!");

    match_test1();

    match_test2();

    match_test3();

    match_test4();
}

// 匹配字面值
fn match_test1 () {
    let x = 5;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("None"),
    }
}

// 匹配命名变量
fn match_test2 () {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y={:?}", y),
        _ => println!("none match"),
    }
}

// 多个模式
fn match_test3 () {
    let x = 1;

    match x {
        1 | 2 => println!("matched one or two"),
        3 => println!("three"),
        _ => println!("none"),
    }
}

// 通过 ..= 匹配值的范围
fn match_test4 () {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("none")
    }

    let y = 'k';

    match y {
        'a'..='c' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}