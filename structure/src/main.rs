struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//tuple structs
fn main() {
    let mut user1 = User {
        email: "ohgree@u.sogang.ac.kr".to_owned(),
        username: "MJ Shin".to_owned(),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("ohgree");

    let mut user2 = build_user("a@a.c".to_string(), "asd".to_string());
    user2.username = String::from("NEW");

    //struct update syntax
    let user3 = User {
        email: String::from("email@some.com"),
        username: String::from("username"),
        ..user1
    };
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    black.0;

    //unit-like structs w/o fields (unit type: () )

}
fn build_user(email: String, username: String) -> User {
    //field init shorthand syntax
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
