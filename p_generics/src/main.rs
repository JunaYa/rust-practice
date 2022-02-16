fn main() {
    println!("Hello, world!");
    p1();

    reg_fn(Single(A));
    gen_fn(SingleGen(A));
    gen_spec_i32(SingleGen(2));

    generic::<char>(SingleGen('a'));

    generic(SingleGen('c'));

    // impletation
    let x = Val { val: 12.9 };
    let gen_val = GenVal { gen_val: 'b' };
    println!("{} {}", x.value(), gen_val.value());

    // trait
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

    // bounds
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    print_debug(&rectangle);
    println!("{}", area(&rectangle));
}

struct A;
struct Single(A);

struct SingleGen<T>(T);

fn p1 () {
    let _s = Single(A);

    let _chat: SingleGen<char> = SingleGen('b');

    let _t  = SingleGen(A);

    let _32 = SingleGen(6);
    let _char = SingleGen('a');
}

fn reg_fn (_s: Single) {}

fn gen_fn (_s: SingleGen<A>) {}

fn gen_spec_i32 (_s: SingleGen<i32>) {}

fn generic<T> (_s: SingleGen<T>) {}

impl SingleGen<f32> {

}
impl SingleGen<A> {
    
}
impl <T> SingleGen<T> {
    
}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl <T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

use std::fmt::Debug;
fn print_debug<T: Debug> (t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea> (t: &T) -> f64 {
    t.area()
}