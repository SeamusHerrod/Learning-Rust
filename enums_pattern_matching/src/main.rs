use rand::{thread_rng, Rng};

fn main() {

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    route(IpAddrKind::V6);
    route(IpAddrKind::V4);
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // this is verbose, and can be written more concisely with enums
    let _home1 = IpAddrAsEnum::V4(127, 0, 0, 1);
    let _loopback1 = IpAddrAsEnum::V6(String::from("::1"));

    let home2 = IpAddrAsEnum1::V4(Ipv4Addr::home());
    dbg!(&home2);

    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let mut y = thread_rng();
    let a: i8 = y.gen();
    let b: i8 = 10;
    let z: Option<i8>;
    println!("a: {}", a);
    if dbg!(a% 2 == 0){
        z = Some(5);
    }
    else {
        z = None;
    }

    dbg!(&z);

    //NOTE: Option<i8> & i8 are distinct types, as such
        // this is illegal:
        //  z + b; == Option<i8> + i8 UNDEFINED
    //NOTE: use match statements in order to extract the i8 value from Option to make this
    //addition valid

    let state = UsState::Alabama;
    let quarter = Coin::Quarter(state);

    println!("value of coin: {}",value_in_cents(quarter));

    // NOTE: we can take special actions for some values and a default actions for others
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // we can also use the catch-all token which is like other but doens't use the value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // if we want NOTHING to happen, then use the unit value '()'
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // we can also use if let to represent a match that matches exactly one case and ignores the
    // rest
    let mut count = 0;
    let state = UsState::Alaska;
    let coin = Coin::Quarter(state);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    let state = UsState::Alaska;
    let coin = Coin::Quarter(state);
    if let Coin::Quarter(state) = coin {
        println!("State quarter form {:?}", state);
    } else {
        count += 1;
    }
}

// enums can only be ONE of its variants or nothing
enum IpAddrKind {
    V4,
    V6,
}
// because V4 and V6 are namespaced under IpAddrKind, they are of the same type, so we can
// define functions that work on any IpAddrKind
fn route(_ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrAsEnum {
    V4(u8,u8,u8,u8),
    V6(String),
}

// even further enums can contain structs as well
#[derive(Debug)]
enum IpAddrAsEnum1 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
struct Ipv4Addr {
    address: String,
}

impl Ipv4Addr {
    fn home() -> Self {
        Self {
            address: String::from("127.0.0.1"),
        }
    }
}
#[derive(Debug)]
struct Ipv6Addr {
    address: String,
}

impl Ipv6Addr {
    fn loopback() -> Self {
        Self {
            address: String::from("::1"),
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

// there is also the Option Enum where a value can be a variant of enum OR nothing
//NOTE: this is how Option is defined in the standard library
// enum Option<T> {
//     None,
//     Some(T),
//}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}: ", state);
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
