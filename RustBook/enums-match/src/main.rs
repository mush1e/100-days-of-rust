#[derive(Debug)]
enum US_state {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(US_state)
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Self::Penny => {
                println!("Lucky Penny!");
                1
            }
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter(state) => {
                println!("this quarter is from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let coin_a = Coin::Penny;
    let coin_b = Coin::Quarter(US_state::Alabama);
    println!("The value of coinA is {} cent", coin_a.value_in_cents());
    println!("CoinB - {} cents", coin_b.value_in_cents());
}
