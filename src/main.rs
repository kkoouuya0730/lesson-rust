// 構造体
fn main() {
    let mut user = User {
        email: String::from("some@example.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };
    user.email = String::from("test@example.com");

    let email = String::from("some@example.com");
    let username = String::from("test");

    let user2 = build_user(email, username);
    let user3 = User {
        email: String::from("some@example.com"),
        username: String::from("test"),
        ..user
    };
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
