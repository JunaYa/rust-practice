fn main() {
    println!("Hello, world!");
    p1();

    reg_fn(Single(A));
    gen_fn(SingleGen(A));
    gen_spec_i32(SingleGen(2));

    generic::<char>(SingleGen('a'));

    generic(SingleGen('c'));

    let x = Val { val: 12.9 };
    let gen_val = GenVal { gen_val: 'b' };
    println!("{} {}", x.value(), gen_val.value());
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