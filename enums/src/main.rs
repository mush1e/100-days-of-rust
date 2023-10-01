// better ish implementation
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// WHOA SO KEWL
enum IpAddr {
    V4(String),
    V6(String),
}

//BRAIN EXPLODE
enum IpAddrGood {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        // do random bullshit
    }
}

fn main() {
    // noob implementation
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //GigaChad
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // United States of Smash
    let home = IpAddrGood::V4(127,0,0,1);
    let loopback = IpAddrGood::V6(String::from("::1"));

    let mut m = Message::Write(String::from("Hello"));
    dbg!(&m);
    m.call();

}
