use crate::score;

fn xor_decrypt(ciphertext: &[u8], key: u8) -> Vec<u8> {
    ciphertext.iter().map(|&b| b ^ key).collect()
}

pub fn decrypt(ciphertext: &[u8]) {
    let (key, text) = (0..=255)
        .filter_map(|key| {
            let decrypted = xor_decrypt(ciphertext, key);
            String::from_utf8(decrypted)
                .ok()
                .map(|text| (key, text.clone(), score::english_bytes(text.as_bytes())))
        })
        .max_by_key(|&(_, _, score)| score)
        .map(|(key, text, _)| (key, text)).unwrap();
    println!("XOR (key 0x{:02x}): {}", key, text);
}
