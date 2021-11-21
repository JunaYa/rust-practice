use lib_slice;

fn main() {
    let s = String::from("hello world");
    let world = lib_slice::first_word(&s[..]);
    println!("Hello, world! {}", world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}