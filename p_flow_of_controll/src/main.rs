fn main() {
    println!("Hello, world!");
    p_if_else();
    p_loop();
    p_nesting_labels();
    p_returning_from_loops();
    p_while();
    p_for_loop();
    p_iter();
    p_into_iter();
    p_iter_mut();
    p_match();
    p_match_tuples();
    p_match_enums();
    p_match_dereference();
    p_match_struct();
    p_match_guard();
    p_match_binding();
    p_if_let();
    p_while_let();
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

fn p_loop () {
    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("count is {}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}

fn p_nesting_labels () {
    let mut count = 0;
    'outer: loop {
        println!("outer loop");

        'inner: loop {
            count += 1;
            println!("inner loop {}", count);
            if count == 2 {
                break 'inner;
            }
            
            if count == 3 {
                break 'outer;
            }
        }
    }

    println!("outer end");
}

fn p_returning_from_loops () {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn p_while () {
    let mut count = 0;

    while count < 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        
        count += 1;
    }
}

fn p_for_loop () {
    for n in 0..3 {
        println!("n is {}", n);
    }

    for m in 0..=3 {
        println!(" m is {}", m);
    }
}

fn p_iter () {
    let list = vec!["a", "b", "c"];
    for item in list.iter() {
        match item {
            &"a" => println!("this is A"),
            _ => println!("item is {}", item)
        }
    }
}

fn p_into_iter () {
    let list = vec!["a", "b", "c"];

    for item in list.into_iter() {
        match item {
            "a" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", item),
        }
    }
}

fn p_iter_mut () {
    let mut list = vec!["a", "b", "c"];
    for item in list.iter_mut() {
        *item = match item {
            &mut "a" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", list);
}

fn p_match () {
    let number = 13;

    match number {
        1 => println!("one"),
        3 | 5 | 7 | 9 | 11 | 13 => println!("This is a prima"),
        13..=40 => println!("A ten"),
        _ => println!("other")
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
        _ => -1,
    };

    println!("binary {}, boolean {}", binary, boolean);
}

fn p_match_tuples () {
    let tripl = (0, 0, 0);

    match tripl {
        (0,  y, z) => println!("x match"),
        (0, ..) => println!("x match"),
        _ => println!("no match"),
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    RGBA(u32, u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn p_match_enums () {
    let color = Color::Blue;
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(r, g, b) => println!("rgb blue"),
        Color::HSV(h, s, v) => println!("hsv"),
        Color::HSL(h, s, l) => println!("hsl"),
        Color::CMY(c, m, y) => println!("cmy"),
        Color::CMYK(c, m , y ,k) => println!("cmyk"),
        _ => println!("no color"),
    }
}

fn p_match_dereference () {
    let reference = &4;

    match reference {
        &val => println!("Got a val {:?}", val),
    }

    match *reference {
        val => println!("Got a val {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;


    match value {
        ref r => println!("Got a reference to a value : {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, `mut_value`: {:?}", m)
        }
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn p_match_struct () {
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo {x: (a, b), y: 2} => println!("Foo a = {}, b = {}", a, b),
        Foo { x: (1, b), y: 1 } => println!("Foo b = {}", b),
        Foo { x: (a, 2), y } => println!("Foo a = {}, y = {}", a, y),
        _ => println!("none"),
    }
}

fn p_match_guard () {
    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom"),
        (x, y) if x % 2 == 0 => println!("The first value is odd"),
        _ => print!("guard"),
    }
}

fn age () -> u32 { 15 }
fn some_number () -> Option<u32> {
    Some(42)
}
fn p_match_binding () {
    match age() {
        n @ 1 ..= 12 => println!("child age is {}", n),
        n @ 13 ..= 19 => println!("child age is = {}", n),
        _ => println!("binding age")
    }

    match some_number() {
        Some(m @ 42) => println!("number is {}", m),
        Some(m) => println!("number is  = {}", m),
        _ => println!("binding some number"),
    }
}

fn p_if_let () {
    let optional = Some(1);
    match optional {
        Some(i) => println!("optional is {:?}", i),
        _ => {}
    }
}


fn p_while_let () {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9");
            optional = None;
        } else {
            println!("i is {}", i);
            optional = Some(i + 1);
        }
    }
}