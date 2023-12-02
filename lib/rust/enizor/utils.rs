pub fn consume_numeral(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .map(|b| (b - b'0') as usize)
        .reduce(|acc, v| acc * 10 + v)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(consume_numeral(b"11"), 11);
    }
}
