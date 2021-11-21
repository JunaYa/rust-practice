fn main() {
    let mut s = String::from("hello world");
    let world = first_word(&s);
    s.clear();
    println!("Hello, world! {}", world);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  i;
        }
    }
    s.len()
}
