
use std::fmt::{ Debug, Display };
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

    // multiple bounds
    let string = "world";
    let array = vec![1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_print(&string);
    compare_types(&array, &vec);

    vec.print_in_option();

    // 
    let number_1 = 1;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("number1 {} number2 {} result {}", 
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("first {}", container.first());
    println!("last {}", container.last());
    println!("diffence {}", difference(&container));
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

fn print_debug<T: Debug> (t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea> (t: &T) -> f64 {
    t.area()
}

fn compare_print<T: Debug + Display> (t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {:?}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

trait PrintInOption {
    fn print_in_option(self);
}

impl <T> PrintInOption for T where Option<T>: Debug {
    fn print_in_option(self) {
        println!("some {:?}", Some(self));
    }
}

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
    container.first() - container.last()
}