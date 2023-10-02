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

fn main() {
    let v4_addr = IpAddrType::V4(127, 0, 0, 1);
    let v6_addr = IpAddrType::V6(String::from("::1"));
    println!("{:#?}", v4_addr.call());
}


