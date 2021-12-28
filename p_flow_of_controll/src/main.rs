fn main() {
    println!("Hello, world!");
    p_if_else();
    p_loop();
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