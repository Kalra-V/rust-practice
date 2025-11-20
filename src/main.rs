fn main() {
    let s1 = String::from("HELLO");
    let s2 = &s1; // s2 is a reference to s1

    let mut s3 = String::from("WORLD"); // mutable reference
    update_str(&mut s3);

    println!("s2: {}", s2);
    println!("s1: {}", s1);
    println!("s3: {}", s3);
}

fn update_str(s: &mut String) {
    s.push_str("!");
}