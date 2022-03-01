fn main() {
    println!("Hello, world!");
    let p1 = Point { x: 1.1f32, y: 1.1f32 };
    let p2 = Point { x: 1.1f32, y: 1.1f32 };

    println!("p1 + p2 = {:?}", p1.add(p2));

    let p3 = Point { x: 1.1f32, y: 1.1f32 };
    let p4 = Point { x: 1.1f32, y: 1.1f32 };

    println!("p3 + p4 = {:?}", add(p3, p4));
}

use std::{ops::Add};
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl <T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>> (a: T, b: T) -> T {
    a + b
}