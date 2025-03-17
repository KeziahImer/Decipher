use std::collections::HashSet;
use std::env;
use std::fs;

fn load_dictionary() -> HashSet<String> {
    let mut dict = HashSet::new();
    if let Ok(content) = fs::read_to_string("ressources/french_words.txt") {
        for word in content.lines() {
            dict.insert(word.to_lowercase());
        }
    }
    dict
}

fn count_valid_words(text: &str, dictionary: &HashSet<String>) -> usize {
    text.split_whitespace()
        .filter(|word| dictionary.contains(&word.to_lowercase()))
        .count()
}

fn caesar_decrypt(ciphertext: &str) -> String {
    let mut best_text = String::new();
    let mut best_score = 0;
    let dictionary = load_dictionary();

    for shift in 1..=25 {
        let decrypted: String = ciphertext.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (c as u8 - first + 26 - shift as u8) % 26) as char
            } else {
                c
            }
        }).collect();

        let word_count = count_valid_words(&decrypted, &dictionary);
        if word_count > best_score {
            best_score = word_count;
            best_text = decrypted;
        }
    }

    best_text
}

fn caesar_encrypt(plaintext: &str, shift: usize) -> String {
    plaintext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            (first + (c as u8 - first + shift as u8) % 26) as char
        } else {
            c
        }
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!("Usage:");
        println!("  Cesar cipher : cargo run cesar c <message> <shift>");
        println!("  Cesar decipher : cargo run cesar d <message>");
        return;
    }
    
    let algo = &args[1];

    if algo == "cesar" {
        let mode = &args[2];
        let message = &args[3];
        match mode.as_str() {
            "c" => println!("Ciphered : {}", caesar_encrypt(message, args[4].parse::<usize>().unwrap())),
            "d" => println!("Deciphered : {}", caesar_decrypt(message)),
            _ => println!("Unknown mode, use c (cipher) or d (decipher)."),
        }
    } else {
        println!("Unknown algorithm.");
    }
}
