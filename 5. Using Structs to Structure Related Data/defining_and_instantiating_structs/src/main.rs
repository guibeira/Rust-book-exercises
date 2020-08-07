fn main() {
    println!("Hello, world!");

    let user1 =  User {
        email: String::from("email@email.com"),
        username: String::from("nomezinho"),
        active:  true,
        sign_in_count: 1,
    };
    println!("email {}", user1.email);

    let user2 = User {
        email: String::from("user2@email.com"),
        username: String::from("Tião"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    
    println!("{}", user2.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
