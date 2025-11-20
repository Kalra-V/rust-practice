fn main() {
    let sentence = String::from("IM VANSH KALRA!");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);
}
 
fn get_first_word(sentence: String) -> String {
    let mut first_word = String::from("");
    for char in sentence.chars() {
        first_word.push(char);
        if char == ' ' {
            break;
        }
    }
    return first_word;
}