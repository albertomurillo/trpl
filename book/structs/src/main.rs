struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sing_in_count: user1.sing_in_count,
    };

    let user1 = build_user(
        String::from("anotheremail@example.com"),
        String::from("someusername123"),
    );

    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 1,
    }
}
