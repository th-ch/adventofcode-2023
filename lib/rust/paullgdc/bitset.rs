#[derive(Debug, Clone, Default)]
pub struct Bitset {
    bits: Vec<u8>,
    len: usize,
}

impl Bitset {
    const BITS: usize = u8::BITS as usize;

    pub fn extend_value(&mut self, mut extra: usize, new: bool) {
        let new_mask = !(new as u8).wrapping_sub(1);
        let offset = self.len % Self::BITS;
        if offset != 0 {
            unsafe {
                *self.bits.get_unchecked_mut(self.len / Self::BITS) |= new_mask << self.len;
            }
            let added = std::cmp::min(Self::BITS - offset, extra);
            extra -= added;
            self.len += added;
        }
        for _ in 0..(extra / Self::BITS) {
            self.bits.push(new_mask);
        }
        if (extra) % Self::BITS != 0 {
            self.bits.push(new_mask);
        }
        self.len += extra;
    }

    pub fn push(&mut self, b: bool) {
        if self.len % Self::BITS == 0 {
            self.bits.push(0);
        }
        self.bits[self.len / Self::BITS] |= (b as u8) << (self.len % Self::BITS);
        self.len += 1;
    }

    pub fn get(&self, i: usize) -> Option<bool> {
        if i >= self.len {
            None
        } else {
            unsafe {
                Some(self.bits.get_unchecked(i / Self::BITS) & (1 << i % Self::BITS) as u8 != 0)
            }
        }
    }

    pub fn set(&mut self, i: usize, b: bool) -> Option<()> {
        if i >= self.len {
            None
        } else {
            unsafe {
                *self.bits.get_unchecked_mut(i / Self::BITS) |= (b as u8) << (i % Self::BITS);
            }
            Some(())
        }
    }

    pub fn all(&self) -> bool {
        let mut res = true;
        for i in 0..(self.len / Self::BITS) {
            res &= unsafe { self.bits.get_unchecked(i) } ^ 0xff == 0
        }
        if self.len % Self::BITS > 0 {
            res &= self.bits.last().unwrap() ^ (1_u8 << self.len % Self::BITS).wrapping_sub(1) == 0;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Bitset;

    #[test]
    fn test_bitset() {
        let mut b = Bitset::default();
        b.push(true);
        b.push(false);
        b.push(true);
        assert_eq!(b.len, 3);

        assert_eq!(b.get(0), Some(true));
        assert_eq!(b.get(1), Some(false));
        assert_eq!(b.get(2), Some(true));
        assert_eq!(b.get(3), None);
        assert_eq!(b.all(), false);

        assert_eq!(b.set(1, true), Some(()));
        assert_eq!(b.get(1), Some(true));
        assert_eq!(b.set(3, true), None);
        assert_eq!(b.all(), true);
    }

    #[test]
    fn test_extend_1() {
        let mut b = Bitset::default();
        b.extend_value(9, true);
        for i in 0..9 {
            assert_eq!(b.get(i), Some(true));
        }
        for i in 9..24 {
            assert_eq!(b.get(i), None);
        }
    }

    #[test]
    fn test_extend_2() {
        let mut b = Bitset::default();
        b.push(false);
        b.extend_value(7, true);
        assert_eq!(b.get(0), Some(false));
        for i in 1..8 {
            assert_eq!(b.get(i), Some(true));
        }
        for i in 8..24 {
            assert_eq!(b.get(i), None);
        }
    }

    #[test]
    fn test_extend_3() {
        let mut b = Bitset::default();
        b.push(false);
        b.extend_value(25, true);
        assert_eq!(b.get(0), Some(false));
        for i in 1..26 {
            assert_eq!(b.get(i), Some(true));
        }
        for i in 26..32 {
            assert_eq!(b.get(i), None);
        }
    }
}
