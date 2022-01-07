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
}
