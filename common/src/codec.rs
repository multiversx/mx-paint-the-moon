pub fn encode_coordinates(x: u32, y: u32) -> u64 {
    ((x as u64) << 32) | (y as u64)
}

pub fn decode_coordinates(encoded: u64) -> (u32, u32) {
    let x = (encoded >> 32) as u32; // Upper 32 bits
    let y = (encoded & 0xFFFFFFFF) as u32; // Lower 32 bits
    (x, y)
} // reversed to big endian because they are little endian by def
