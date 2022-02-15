fn main() {
    println!("Hello, world!");

    used_function();

    are_you_on_linux();

    if cfg!(target_os = "linux") {
        println!("are Linux");
    } else {
        println!("are not Linux");
    }
}

fn used_function () {}

#[allow(dead_code)]
fn unused_function () {}

#[cfg(target_os = "linux")]
fn are_you_on_linux () {
    println!("is Linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux () {
    println!("is not Linux")
}