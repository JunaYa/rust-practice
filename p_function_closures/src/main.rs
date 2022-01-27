fn main() {
   
    init();
    capturing();
    input_paramaters();
    type_anonymity();
    input_function();
}

fn init () {
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

fn input_paramaters () {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said greeting {}", greeting);
        farewell.push_str("!!!");
        println!("Then I am screamed {}", farewell);
        println!("ZZzz");
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled {}", apply_to_3(double)); 
}

fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) ->i32 {
    f(3)
}

fn type_anonymity () {
    let x = 7;
    let print = || println!("{}", x);

    apply_fn(print);
}

fn apply_fn<F>(f: F) where F:Fn() {
    f();
}

fn input_function () {
    let closure = || println!("I am a closure");
    call_me(closure);
    call_me(function);
}

fn function() {
    print!("i am a function");
}

fn call_me<F: Fn()>(f: F) {
    f();
}