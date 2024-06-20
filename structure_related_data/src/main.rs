struct User {
    active: bool, // use Copy trait so no move of the data
    username: String,
    email: String,
    sign_in_count: u64, // this type use Copy trait, so no movement of data
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("testuser"),
        email: String::from("testuser@gmail.com"),
        sign_in_count: 1,
    };

    let user3 = User {
        email: String::from("user3@gmail.com"),
        ..user1
    };

    let email = String::from("newtestuser@gmail.com");
    let username = String::from("newtestuser");

    let user2 = build_user(email, username);
    println!("username of user2 is {}", user2.username);
}
