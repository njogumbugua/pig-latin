fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first_char) => {
            if is_vowel(first_char) {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", chars.collect::<String>(), first_char)
            }
        }
        None => format!("string is empty"),
    }
}
fn main() {
    let input = String::from("first apple");
    let mut output = String::new();

    for word in input.split_whitespace() {
        output.push_str(&pig_latin(word));
        output.push(' ');
    }

    println!("{}", output.trim());
}
