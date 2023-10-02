enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value_in_cents (&self) -> u8 {
        match self {
            Self::Penny => {
                println!("Lucky penny!");
                1
            }
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25
        }
    } 
}

fn main() {
    let coin_a = Coin::Penny;
    println!("The value of coinA is {} cent", coin_a.value_in_cents());
}
