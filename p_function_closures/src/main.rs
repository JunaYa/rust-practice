use std::vec;

fn main() {
   
    init();
    capturing();
    input_paramaters();
    type_anonymity();
    input_function();
    as_output_parameters();

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    println!("2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];
    // println!("2 in array1: {}", arr1.iter()     .any(|&x| x == 2));
    // println!("2 in array2: {}", arr2.into_iter().any(| x| x == 2));
    println!("2 in array1: {:?}", arr1.iter()     .find(|&&x| x == 2));
    println!("2 in array2: {:?}", arr2.into_iter().find(|&x| x == 2));
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

fn as_output_parameters() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a : {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a : {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a : {}", text)
}