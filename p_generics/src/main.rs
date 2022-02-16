fn main() {
    println!("Hello, world!");
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

