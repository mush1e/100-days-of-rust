struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user (email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 0
    }
}

fn main() {
    let username = String::from("Mu5h1e");
    let email = String::from("abc");
    let mut user1 = build_user(&email, &username);

    user1.email.push_str(" hello there");
    println!("{}", user1.email); 
}
