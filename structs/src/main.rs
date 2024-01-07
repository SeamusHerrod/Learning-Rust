use std::io;
fn main() {
    let _user1 = User {
        active: true,
        username: String::from("sharkbait"),
        email: String::from("smh@gmail.com"),
        sign_in_count: 1,
    };
    // instances may also be mutable so that its fields can be changed
    // let mut user1 = User { ... }
    let mut _new_user = make_user();
    // use . notation to access fields
    let mut user_email = _new_user.email;
    println!("_new_user email: {}", user_email);
    _new_user.email = String::from("shermy@gmail.com");
    println!("_new_user email: {}", _new_user.email);

    // we can create a new instance of a struct that changes some fields but not others
    // Struct Update Syntax
    let user2 = User {
        email: String::from("grundle@gmail.com"),
        .._user1
    };

    // NOTE: even though the fields of both Color and Point are both i32, they are of their own
    // type, for example
        // black.0 = point.0 is NOT allowed
    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // printing structs for debugging can be accomplished using the debug mode {:?} in println!
    // NOTE: this is true ONLY true if the struct derives or implements Debug
    let rect = Rectangle {
        width: 12,
        height: 7,
    };
    println!("rectange: {:?}", rect);
    let area = area(&rect);
    println!("area: {}", area);
    // NOTE: dbg! takes ownership of an expression, prints file and line # where dbg! is called
    // to stderr
    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    let rect1_area = dbg!(rect1.area());
    rect1.change_width(10);
    dbg!(&rect1);
    rect1.change_width(-80);
    dbg!(&rect1);
    rect1.change_width(-45);
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 20,
        height: 35,
    };
    let rect3 = Rectangle {
        width: 80,
        height: 60,
    };

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Cant rect1 hold rect3 {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(12);
    println!("{:?}", sq);
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
        //field init notation allows use to not rewrite username, email, ...
        //IF the passed parameters have the same name
        username,
        email,
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

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// adding method to Rectangle
// everything that is added within the impl block will be associated with Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn change_width(&mut self, change: i32){
        let change_as_u32 = i32::abs(change) as u32;
        println!("change_as_u32: {}", change_as_u32);
        if change < 0 {

            if dbg!(change_as_u32 > self.width){
                println!("negative width is not allowed");
            }
            else {
                self.width -= change_as_u32;
            }
        }
        else {
            self.width += change as u32;
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if other.width < self.width && other.height < self.height {
            true
        }
        else {
            false
        }
    }
    // this is an ASSOCIATED function, functions a part of a structs impl that don't take &self
    // as a parameter
    // NOTE: associated functions are often used for constructors because they return an instance
    // of the struct
    // NOTE: associated functions are called using '::' syntax
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// we can also add methods to structs as opposed to just writing functions like this
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width *  rectangle.height
}
