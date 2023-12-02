pub fn consume_numeral(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .map(|b| (b - b'0') as usize)
        .sum()
}
