fn main() {
    println!("Hello, world!");
    let my_name =  "Pascal";
    // great(my_name);

    // slice
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[2..s.len()];
    let slice = &s[..];
}

fn great(name: String) {
    println!("string is {}", name);
}
