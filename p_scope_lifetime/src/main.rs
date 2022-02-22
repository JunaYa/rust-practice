fn main() {
    println!("Hello, world!");

    let (x, y) = (2, 3);
    print_refs(&x, &y);

    println!("out x is {}", &x);

    borrow_error();

    let x = 1;
    let mut y = 1;
    print_one(&x);

    add_one(&mut y);
    println!("add one after y is {}", y);

    print_one(&y);
    
    let mut owner = Owner(1);
    owner.print();
    owner.add();
    owner.print();

    p_struct();
}

fn print_refs<'a, 'b> (x: &'a i32, y: &'b i32) {
    println!("x is {} y is {}", x, y);
}

fn borrow_error<'a> () {
    let _x = 2;
    // let y: &'a i32 = &_x;
}

fn print_one<'a> (x: &'a i32) {
    println!("print one x is {}", x);
}

fn add_one<'a> (x: &'a mut i32) {
    *x += 1;
}

struct Owner(i32);
impl Owner {
    fn add<'a> (&'a mut self) { self.0 += 1}
    fn print<'a> (&'a self) {
        println!("owner is {}", self.0);
    }
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NameBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn p_struct () {
    let x = 12;
    let y = 14;

    let single = Borrowed(&x);
    let double = NameBorrowed {x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("single is {:?}", single);
    println!("double is {:?} {} {}", double, double.x, double.y);
    println!("reference is {:?}", reference);
    println!("numer is {:?}", number);
}

