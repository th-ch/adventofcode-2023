pub fn consume_numeral(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .map(|b| (b - b'0') as usize)
        .reduce(|acc, v| acc * 10 + v)
        .unwrap_or(0)
}

/// beat a (t,d) race while pressing for `x ms` iff (time spent travelling)*speed > d
/// <=> `(t-x)x > d`
/// Look at the roots of `-x² + tx -d` :
/// `delta = sqrt(t²-4*d)`
/// R1 = first integer > first root is `floor((t-delta)/2) + 1`
/// R2 = first integer < second root is `ceil((t+delta)/2) -1`
/// number of integers between the roots is `1 + R2 - R1`
/// Cannot find a way to reduce the formula :(
/// Float precision issues: ht + hdelta must be exact when expecting an integer,
/// ht is either .0 or .5 => exact for ht & hdelta < 2^52 => exact for ht < 2^51
/// => t < 2^52
pub fn beat_race(t: f64, d: f64) -> usize {
    debug_assert!(t < (1u64 << 52) as f64);
    let ht = t / 2.;
    let hdelta = (ht * ht - d).sqrt();
    (ht + hdelta).ceil() as usize - 1 - (ht - hdelta).floor() as usize
}

pub fn debug_ascii(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(consume_numeral(b"11"), 11);
    }
}
