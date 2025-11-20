struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Kin"),
        email: String::from("vanshkalra645@gmail.com"),
        sign_in_count: 13321312,
    };

    println!("{}'s email is {}", user1.username, user1.email);
}
