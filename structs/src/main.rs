use std::io;
fn main() {
    let user1 = User {
        active: true,
        username: String::from("sharkbait"),
        email: String::from("smh@gmail.com"),
        sign_in_count: 1,
    };
    // instances may also be mutable so that its fields can be changed
    // let mut user1 = User { ... }
    let new_user = make_user();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,

    }
   }

fn make_user() -> User {
    let mut username = String::new();
    let mut email = String::new();

    println!("Enter your username");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    println!("Enter your email");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    build_user(email, username)
}
