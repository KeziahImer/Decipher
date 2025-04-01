// Score text based on English letter frequency (higher = better match)
pub fn english(text: &str) -> u32 {
    let freq = "etaoin shrdlu";
    text.chars().filter(|c| freq.contains(*c)).count() as u32
}

// Improved XOR decryption with better scoring
pub fn english_bytes(bytes: &[u8]) -> u32 {
    let common = b"etaoinshrdlu ETAOINSHRDLU";
    bytes.iter().filter(|&b| common.contains(b)).count() as u32
}
