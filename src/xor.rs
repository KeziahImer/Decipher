use std::collections::HashMap;

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16)
             .map_err(|e| e.to_string()))
        .collect()
}

fn xor_with_key(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    ciphertext.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

fn score_text_french(text: &[u8]) -> f64 {
    // French letter frequencies (approximate)
    // Source: https://fr.wikipedia.org/wiki/Fr%C3%A9quence_d%27apparition_des_lettres_en_fran%C3%A7ais
    let freq = HashMap::from([
        (b' ', 0.15),  // Space is more frequent in French
        (b'e', 0.1446), (b'a', 0.0843), (b'i', 0.0758), (b's', 0.0758),
        (b'n', 0.0712), (b't', 0.0706), (b'r', 0.0665), (b'o', 0.0546),
        (b'l', 0.0538), (b'u', 0.0495), (b'd', 0.0438), (b'c', 0.0344),
        (b'm', 0.0300), (b'p', 0.0297), (b'g', 0.0199),
        (b'b', 0.0147), (b'v', 0.0127), (b'h', 0.0121), (b'f', 0.0119),
        (b'q', 0.0115), (b'y', 0.0113), (b'x', 0.0045), (b'j', 0.0034),
        (b'k', 0.0023),
        (b'w', 0.0019), (b'z', 0.0017),
    ]);
    
    text.iter()
        .filter_map(|&c| {
            freq.get(&c)
        })
        .sum()
}

fn crack_xor(ciphertext: &[u8]) -> (Vec<u8>, Vec<u8>, f64) {
    let mut best_key = Vec::new();
    let mut best_plaintext = Vec::new();
    let mut best_score = f64::MIN;
    
    // Try single-byte keys first (most common)
    for key_len in 1..=32 {
        let mut key = vec![0u8; key_len];
        
        // For each position in the key
        for i in 0..key_len {
            let mut best_byte_score = f64::MIN;
            let mut best_byte = 0u8;
            
            // Try all possible byte values
            for byte in 0..=255 {
                let mut score = 0.0;
                let mut count = 0;
                
                // Only look at bytes encrypted with this key byte
                for j in (i..ciphertext.len()).step_by(key_len) {
                    let decrypted = ciphertext[j] ^ byte;
                    
                    // Check if it's an ASCII character (including extended for French)
                    if decrypted.is_ascii() || (decrypted >= 0xC0 && decrypted <= 0xFF) {
                        score += 1.0;
                    }
                    count += 1;
                }
                
                if count > 0 {
                    let normalized_score = score / count as f64;
                    if normalized_score > best_byte_score {
                        best_byte_score = normalized_score;
                        best_byte = byte;
                    }
                }
            }
            
            key[i] = best_byte;
        }
        
        let plaintext = xor_with_key(ciphertext, &key);
        let score = score_text_french(&plaintext);
        
        if score > best_score {
            best_score = score;
            best_key = key;
            best_plaintext = plaintext;
        }
    }
    
    (best_key, best_plaintext, best_score)
}

pub fn decrypt(argtext: &str) -> (Vec<u8>, Vec<u8>, f64) {
    let hex_ciphertext = argtext.trim();
    let test: Vec<u8> = hex_ciphertext.as_bytes().to_vec();
    let restest: Vec<u8> = hex_ciphertext.as_bytes().to_vec();
    
    match hex_to_bytes(hex_ciphertext) {
        Ok(ciphertext) => {
            let (key, plaintext, score) = crack_xor(&ciphertext);
            
            return (key, plaintext, score);
        },
        Err(e) => return (test, restest , 0.0),
    }
}
