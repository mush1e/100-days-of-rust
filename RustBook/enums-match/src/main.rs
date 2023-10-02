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

fn add_one (x : Option <i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn add_two (x : Option<i32>) -> Option<i32>{
    if let Some(i) = x {
        println!("{}", i+2);
        return Some(i+2);
    }
    None
}

fn main() {
    let coin_a = Coin::Penny;
    let coin_b = Coin::Quarter(US_state::Alabama);
    println!("The value of coinA is {} cent", coin_a.value_in_cents());
    println!("CoinB - {} cents", coin_b.value_in_cents());

    let five = Some(5);
    let six = add_one(five);
    let seven = add_two(five);
    let none = add_one(None);
}
