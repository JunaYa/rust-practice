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