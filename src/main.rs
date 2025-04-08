mod score;
mod caesar;
mod xor;
use std::env;
use std::error::Error;
use hex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <auto|caesar|xor> <ciphertext>");
        return Ok(());
    }

    let mode = &args[1];
    let ciphertext = &args[2];
        
    match mode.as_str() {
        "auto" => {
            if ciphertext.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()) {
                caesar::decrypt(ciphertext)
            } else {
                let bytes = hex::decode(ciphertext.replace(" ", ""))?;
                xor::decrypt(&bytes)
            }
        },
        "caesar" => {
            caesar::decrypt(ciphertext);
        },
        "xor" => {
            let bytes = hex::decode(ciphertext.replace(" ", ""))?;
            xor::decrypt(&bytes);
        },
        _ => eprintln!("Usage: cargo run <auto|caesar|xor> <ciphertext> [--hex]"),
    }

    Ok(())
}
