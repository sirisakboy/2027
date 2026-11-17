// K-Line 10400 baud implementation
pub struct KLine;

impl KLine {
    pub fn checksum(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
    }
}
