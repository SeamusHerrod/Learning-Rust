fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V6);
    route(IpAddrKind::V4);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // this is verbose, and can be written more concisely with enums
    let home1 = IpAddr_as_enum::V4(127, 0, 0, 1);
    let loopback1 = IpAddr_as_enum::V6(String::from("::1"));

    let home2 = IpAddr_as_enum1::V4(Ipv4Addr::home());
    dbg!(&home2);

    let m = Message::Write(String::from("hello"));
    m.call();
}

// enums can only be ONE of its variants or nothing
enum IpAddrKind {
    V4,
    V6,
}
// because V4 and V6 are namespaced under IpAddrKind, they are of the same type, so we can
// define functions that work on any IpAddrKind
fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr_as_enum {
    V4(u8,u8,u8,u8),
    V6(String),
}

// even further enums can contain structs as well
#[derive(Debug)]
enum IpAddr_as_enum1 {
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
