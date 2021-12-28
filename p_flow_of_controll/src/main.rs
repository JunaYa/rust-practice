fn main() {
    println!("Hello, world!");
    p_if_else();
    p_loop();
    p_nesting_labels();
    p_returning_from_loops();
    p_while();
    p_for_loop();
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