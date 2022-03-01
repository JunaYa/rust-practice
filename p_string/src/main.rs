fn main() {
    println!("Hello, world!");
    let my_name =  "Pascal";
    // great(my_name);

    // slice
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[2..s.len()];
    let slice = &s[..];
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

fn great(name: &String) {
    println!("string is {}", name);
}

fn display_array <T: std::fmt::Debug, const N: usize> (arr: [T; N]) {
    println!("{:?}", arr);
}