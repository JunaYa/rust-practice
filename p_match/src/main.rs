fn main() {
    println!("Hello, world!");

    match_test1();

    match_test2();

    match_test3();

    match_test4();

    match_test5();

    match_test6();

    match_test7();
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


struct Point {
    x: i32,
    y: i32,
}
// 解构并分解值
fn match_test5 () {
    let p = Point { x: 1, y: 1};

    match p {
        Point {x, y: 1} => println!("x at {}", x),
        Point {x: 0, y} => println!("y at {}", y),
        Point { x, y } => println!("x = {}, y = {}", x, y),
    }
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}
// 解构枚举
fn match_test6 () {
    // let msg = Message::ChangeColor(Color::RGB(255, 255, 255));
    let msg = Message::ChangeColor(Color::HSV(255, 255, 255));

    match msg {
        Message::Quit => {
            println!("quit");
        },
        Message::Move {x, y} => {
            println!("x = {}, y = {}", x, y);
        },
        Message::Write(text) => {
            println!("write tesx = {}", text);
        },
        Message::ChangeColor(Color::RGB(r, g, b)) => {
            println!("ChangeColor r = {}, g = {}, b = {}", r, g, b);
        },
        Message::ChangeColor(Color::HSV(h, s, v)) => {
            println!("ChangeColor h = {}, s = {}, v = {}", h, s, v);
        }
    }
}

// 忽略一些
fn match_test7 () {
    // 忽略未绑定的值
    
    let s = Some(String::from("Hello!"));
    
    // 单纯下滑线不绑定值
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 以下划线开头的未使用变量仍然会绑定值
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // 它可能会获取值的所有权
    // println!("{:?}", s);
}