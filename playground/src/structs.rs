#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn build_user(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }
}

// tuple struct
struct Color(i32, i32, i32);

fn main() {
    let mut user_1 = User {
        username: String::from("Mush1e"),
        email: String::from("Lul"),
        sign_in_count: 0,
        active: true,
    };
    user_1.username = String::from("Mu5h1e");
    let user_2 = User::build_user("Mustafa".to_string(), "abc@123.com".to_string());
    println!("{:?}\n{:?}", user_1, user_2);
    let user_3 = User {
        username: String::from("abc"),
        ..user_1
    };
    println!("{:?}", user_3);
}
