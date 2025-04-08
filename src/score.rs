pub fn english(text: &str) -> u32 {
    let freq = "etaoin shrdlu";
    text.chars().filter(|c| freq.contains(*c)).count() as u32
}

pub fn english_bytes(bytes: &[u8]) -> u32 {
    let common = b"etaoinshrdlu ETAOINSHRDLU";
    bytes.iter().filter(|&b| common.contains(b)).count() as u32
}
