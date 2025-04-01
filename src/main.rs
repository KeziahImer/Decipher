mod score;
use std::env;
use std::error::Error;
use hex;

// Brute-force all Caesar shifts
fn brute_force_caesar(ciphertext: &str) {
    println!("Brute-forcing Caesar cipher:");
    for shift in 1..=26 {
        println!("Shift {:2}: {}", shift, caesar_decrypt(ciphertext, shift));
    }
}

// Brute-force single-byte XOR (with ASCII heuristic)
fn brute_force_xor(ciphertext: &[u8]) {
    println!("Brute-forcing XOR cipher:");
    for key in 0..=255 {
        let decrypted = xor_decrypt(ciphertext, key);
        let text = String::from_utf8_lossy(&decrypted);
        // if let Ok(text) = String::from_utf8(decrypted) {
        //     if text.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) {
                println!("Key {:03}: {}", key, text);
        //     }
        // }
    }
}

// Caesar Decrypt (now with frequency analysis)
fn caesar_decrypt(ciphertext: &str, shift: u8) -> String {
    ciphertext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = base + (c as u8 - base + 26 - shift as u8) % 26;
            (base + shifted) as char
        } else {
            c
        }
    }).collect()
}



// Auto-decrypt Caesar by finding the shift with the best English score
fn auto_caesar(ciphertext: &str) -> Option<(u8, String)> {
    (0..26).map(|shift| {
        let decrypted = caesar_decrypt(ciphertext, shift);
        let score = score::english(&decrypted);
        (shift, decrypted, score)
    }).max_by_key(|&(_, _, score)| score)
    .map(|(shift, text, _)| (shift, text))
}

// XOR Decrypt (single-byte key)
fn xor_decrypt(ciphertext: &[u8], key: u8) -> Vec<u8> {
    ciphertext.iter().map(|&b| b ^ key).collect()
}

fn auto_xor(ciphertext: &[u8]) -> Option<(u8, String)> {
    (0..=255)
        .filter_map(|key| {
            let decrypted = xor_decrypt(ciphertext, key);
            String::from_utf8(decrypted)
                .ok()
                .map(|text| (key, text.clone(), score::english_bytes(text.as_bytes())))
        })
        .max_by_key(|&(_, _, score)| score)
        .map(|(key, text, _)| (key, text))
}

// CLI Handler
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: cargo run <auto|caesar|xor> <ciphertext> [--hex]");
        return Ok(());
    }

    let mode = &args[1];
    let ciphertext = &args[2];
    let is_hex = if args.len() == 4 {&args[3] == "--hex"} else {false};

    let bytes = if is_hex {
        hex::decode(ciphertext)?
    } else {
        ciphertext.as_bytes().to_vec()
    };

    match mode.as_str() {
        "auto" => {
            if ciphertext.chars().all(|c| c.is_ascii_alphabetic()) {
                if let Some((shift, text)) = auto_caesar(ciphertext) {
                    println!("Caesar (shift {}): {}", shift, text);
                }
            } else {
                if let Some((key, text)) = auto_xor(&bytes) {
                    println!("XOR (key 0x{:02x}): {}", key, text);
                }
            }
        },
        "caesar" => {
            brute_force_caesar(ciphertext);
        },
        "xor" => {
            brute_force_xor(&bytes);
        },
        _ => eprintln!("Invalid mode. Use 'auto', 'caesar', or 'xor'."),
    }

    Ok(())
}
