#[derive(Debug)]
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddrType {
    fn call(&self) -> String {
        String::from("Made call")
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // println!("{}", self.Write);
    }
}

fn main() {
    let v4_addr = IpAddrType::V4(127, 0, 0, 1);
    let v6_addr = IpAddrType::V6(String::from("::1"));

    let m = Message::Write(String::from("Poggy woggy"));
    m.call();

    println!("{:#?}", v4_addr.call());
}


