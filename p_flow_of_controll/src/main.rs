fn main() {
    println!("Hello, world!");
    p_if_else();
    p_loop();
    p_nesting_labels();
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
