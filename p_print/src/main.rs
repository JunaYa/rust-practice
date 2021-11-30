fn main() {
    println!("Hello, world!");

    println!("{} days", 31i64);
    // 31 days

    println!("{} days", 31);
    // 31 days

    // 作为索引使用
    println!("{0}, this is {1}, this is {2}", "A", "B", "C");
    // A, this is B, this is C

    // 别名使用
    println!("{a}, {b}, {c}",
        a="A",
        b="B",
        c="C",
    );
    // A, B, C

    println!("{} of {:b}", 1, 2);
    // 1 of 10

    println!("{number:>width$}", number=1, width=10);
    //         1

    println!("{number:#>width$}", number=1, width=10);
    // #########1

    // fommatted print
    let aya = Person {
        name: "aya",
        age: 12,
    };
    println!("person is {:?}", aya);
    // person is Person { name: "aya", age: 12 }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}