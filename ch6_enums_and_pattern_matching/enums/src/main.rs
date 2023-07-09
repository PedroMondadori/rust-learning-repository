#![allow(unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn basic_enum_test() {
    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    dbg!(home);
    dbg!(loopback);
}

#[derive(Debug)]
enum IpAddrWithData {
    V4(String),
    V6(String),
}

fn with_data_enum_test() {
    let home: IpAddrWithData = IpAddrWithData::V4(String::from("127.0.0.1"));
    let loopback: IpAddrWithData = IpAddrWithData::V6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn option_enum_study() {
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;

    // The code bellow won't compile because you can't add i8 to Option<i8>
    // y could potentially be None, so the compiler won't let us add it to x.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum: i8 = x + y;
}

fn main() {
    basic_enum_test();
    option_enum_study();
    with_data_enum_test();
}
