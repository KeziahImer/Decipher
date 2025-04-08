use crate::score;

fn caesar_decrypt(ciphertext: &str, shift: u8) -> String {
    return ciphertext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = base + (c as u8 - base + 26 - shift as u8) % 26;
            return shifted as char
        } else {
            return c
        }
    }).collect();
}

pub fn decrypt(ciphertext: &str) {
    let (shift, text) = (0..26).map(|shift| {
        let decrypted = caesar_decrypt(ciphertext, shift);
        let score = score::english(&decrypted);
        (shift, decrypted, score)
    }).max_by_key(|&(_, _, score)| score)
    .map(|(shift, text, _)| (shift, text)).unwrap();
    println!("Caesar (shift {}): {}", shift, text);
}
