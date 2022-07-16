//
// Receives string from stdin, prints it pig-latin encoded
//

use std::io::stdin;

fn main() {
    loop {
        let mut txt = String::new();
        match stdin().read_line(&mut txt) {
            Ok(0) => break,
            Ok(..) => {
                for word in txt.split_whitespace() {
                    pig_latinify(word);
                }
            },
            Err(_) => break,
        }
    }
}

fn pig_latinify(word: &str) {

    let mut new_word = String::from(word);
    let replace_char = match word.chars().next() {
        Some(chair) => {
            if is_vowel(chair) {
                'h'
            } else {
                new_word.remove(0);
                chair
            }
        },
        None => {
            println!("Empty string!");
            return;
        },
    };
    println!("{}-{}ay", new_word, replace_char);
}

fn is_vowel(chair: char) -> bool {
    let vowels = "aáâäãeéêëẽiíîïĩoóôöõuúûüũ";
    for vowel in vowels.chars() {
        if chair == vowel {
            return true
        }
    }
    false
}
