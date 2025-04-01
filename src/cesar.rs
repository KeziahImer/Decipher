mod score;
use std::collections::HashSet;
use std::fs;

fn load_dictionary() -> HashSet<String> {
    let mut dict = HashSet::new();
    if let Ok(content) = fs::read_to_string("ressources/french_words.txt") {
        for word in content.lines() {
            dict.insert(word.to_lowercase());
        }
    }
    return dict;
}

fn count_valid_words(text: &str, dictionary: &HashSet<String>) -> usize {
    text.split_whitespace()
        .filter(|word| dictionary.contains(&word.to_lowercase()))
        .count()
}

pub fn decrypt(ciphertext: &str) -> (String, usize) {
    let mut best_text = String::new();
    let mut best_score = 0;
    let dictionary = load_dictionary();

    for shift in 1..=25 {
        let decrypted: String = ciphertext.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                return (first + (c as u8 - first + 26 - shift as u8) % 26) as char;
            } else {
                return c;
            }
        }).collect();

        let word_count = count_valid_words(&decrypted, &dictionary);
        if word_count > best_score {
            best_score = word_count;
            best_text = decrypted;
        }
    }
    return (best_text, best_score);
}