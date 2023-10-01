// Generic ass struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(u32, u32, u32);

// Unit structs
struct isValid;

// Build struct function mmm very naic :D
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
    let user1 = build_user(&email, &username);

    let black = Color(0, 0, 0);


    println!("{}", user1.email); 

    // Regular boring ass syntax
    let mut user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("bar"),
        sign_in_count: user1.sign_in_count,
    };
    user2.active = false;
    println!("{}", user2.active);


    // Cool struct update syntax
    let cool_user = User {
        username: String::from("CoolGigaChad"),
        ..user2
    };

    println!("{} is an absolute gigachad", cool_user.username);
    println!("{} is a poopy user", user2.username);
    println!("{} is the rgb for black", black.0);
}
