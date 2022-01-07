fn main() {
    println!("Hello, world!");

    fn function (i: i32) -> i32 { i + 2 }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i  = 1;

    println!("function {}", function(i));
    println!("closures_annotated {}", closure_annotated(i));
    println!("closures_inferred {}", closure_inferred(i));

    let one = || i;

    println!("one {}", one());

    capturing();
}

fn capturing () {
    use std::mem;

    let color = String::from("green");
    let print = || println!("color: {}", color);
    print();

    let _reborrow = &color;
    print();

    let _color_move = color;
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count {}", count);
    };

    inc();

    inc();

    let _count_reborrow = &mut count;

    let movable = Box::new(2);

    let consume = || {
        println!("movable - {}", movable);
        mem::drop(movable);
    };

    consume();

    // consume();

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&3));
    println!("{}", contains(&1));
    
}
