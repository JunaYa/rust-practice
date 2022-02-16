fn main() {
    println!("Hello, world!");
    p1();

    reg_fn(Single(A));
    gen_fn(SingleGen(A));
    gen_spec_i32(SingleGen(2));

    generic::<char>(SingleGen('a'));

    generic(SingleGen('c'));
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


