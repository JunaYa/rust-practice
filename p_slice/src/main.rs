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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_first_word() {
        let s = String::from("hello world");
        assert_eq!("hello", first_word(&s))
    }
}