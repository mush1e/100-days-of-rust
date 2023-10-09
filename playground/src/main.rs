enum IPAddrKind {
    V4(String),
    V6(String),
}

struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

struct Message;

impl Message {
    fn call(&self) {
        println!("lmao get rekt");
    }
}

enum Coin {
    penny,
    nickel,
    dime,
    quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::penny => 1,
        Coin::nickel => 5,
        Coin::dime => 10,
        Coin::quarter => 25,
    }
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    // dis da noob wae to do shit
    // let localhost = IPAddr {
    //     kind: IPAddrKind::V4,
    //     address: String::from("127.0.0.1");
    // }

    // da gigachad way
    let localhost = IPAddrKind::V4(String::from("127.0.0.1"));

    let msg = Message;
    msg.call();

    let coin = Coin::quarter;
    println!("{}", value_in_cents(coin));
}
