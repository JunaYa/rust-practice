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

    // minmax
    let minmax = Minmax(12, 12);
    println!("custom minmax -----");
    println!("Display {}", minmax);
    // Display 12, 12
    println!("Debug {:?}", minmax);
    // Debug Minmax(12, 12)

    // list test
    let v = List(vec![1, 2, 3]);
    println!("Display {}", v);
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}

// -------------------std::fmt

use std::fmt::{self, write};

#[derive(Debug)]
struct Minmax(i64, i64);

impl fmt::Display for Minmax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?
            }
            write!(f, "{}", v)?
        }
        write!(f, "]")
    }
}