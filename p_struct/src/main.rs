use std::sync::mpsc::Receiver;

fn main() {

//    runP1();

    runP2();

}

fn runP1() {
    let mut user1 = User {
        email: String::from("ayac3j@gmail.com"),
        username: String::from("aya"),
        active: true,
        sign_in_count: 1,
    };

    user1.active = false;
    println!("Hello, world! {:?}", user1);

    let mut user2 = build_user(String::from("ayacj3"), String::from("ayay"));
    println!("user2 is {:?}", user2);
    user2.active = true;
    println!("user2 is {:?}", user2);

    let user3 = User {
        username: String::from("jun"),
        ..user1
    };

    user1.email = String::from("a@gmail.com");
    println!("Hello, world! {:?}", user3);
    println!("Hello, world! {:?}", user1.email);
}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User { username: username, email: email, sign_in_count: 1, active: false }
}

fn runP2 () {
    let width1 = 30;
    let height1 = 50;
    let area_value1 = area(width1, height1);
    let area_value2 = area_by_tupl((width1, height1));
    let scale = 2;
    let rect1 = Rectangle { width: dbg!(30 * scale), height: 50 };
    println!("The area of the rectangle is {} square pixels", rect1.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_tupl (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area_by_struct (rect: Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A Unit
struct Unit;

// A tuple 
struct Pair(u32, u32);