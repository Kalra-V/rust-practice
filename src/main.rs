use std::fs;

fn main () {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(string) => {
            println!("READING: {}", string)
        },
        Err(error) => {
            println!("ERROR found: {}", error)
        }
    }

    print!("works")
}