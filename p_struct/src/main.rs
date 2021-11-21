fn main() {

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