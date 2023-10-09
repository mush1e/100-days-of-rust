#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user_1 = User {
        username: String::from("Mush1e"),
        email: String::from("Lul"),
        sign_in_count: 0,
        active: true,
    };
    user_1.username = String::from("Mu5h1e");
    println!("{:?}", user_1);
}
