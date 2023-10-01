struct IpAddrV {
    V4 : (u8, u8, u8, u8),
    V6 : String
}

impl IpAddrV {
    fn V4(x: u8, y: u8, z: u8, a: u8) -> Self {
        IpAddrV {
            V4: (x, y, z, a),
            V6: "".to_string()
        }
    }

    fn display_v4 (&self) -> String {
        self.V4.0.to_string()
    }
}




fn main() {
    let ip_addr = IpAddrV::V4(127,0,0,1);
    println!("{}" ,ip_addr.display_v4());
}
