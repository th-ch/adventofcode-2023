use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

/// Supports labels up to 7 letters
/// high 56 gits is the label (ASCII value)
/// low 8 bits is the focal length
/// a value of 0 for both is a removed lens
#[derive(Clone, Copy, Default)]
struct Lens(u64);
const FOCAL_BITS: u64 = 8;
const FOCAL_MASK: u64 = (1 << FOCAL_BITS) - 1;

impl Lens {
    fn new(label: u64, focal: u64) -> Self {
        Self(label << FOCAL_BITS | focal)
    }
    fn focal_length(&self) -> u64 {
        self.0 & FOCAL_MASK
    }
    fn label(&self) -> u64 {
        self.0 >> FOCAL_BITS
    }
    fn mark_invalid(&mut self) {
        self.0 = 0;
    }
    fn is_valid(&self) -> bool {
        self.0 != 0
    }
    fn set_focal(&mut self, focal: u64) {
        self.0 &= !FOCAL_MASK;
        self.0 |= focal;
    }
}

// Assumes that for a given box, we never inserts more than that
const MAX_LENSES: usize = 24;

/// A collection of lenses
/// A given label must never appear more than 1 time
#[derive(Clone, Copy, Default)]
struct Box {
    lenses: [Lens; MAX_LENSES],
    len: usize,
}

impl Box {
    fn focusing_power(&self) -> u64 {
        let mut res = 0;
        let mut pos = 1;
        for lens in &self.lenses[..self.len] {
            if lens.is_valid() {
                res += pos * lens.focal_length();
                pos += 1;
            }
        }
        res
    }
    fn remove(&mut self, label: u64) {
        for l in &mut self.lenses[..self.len] {
            if l.label() == label {
                l.mark_invalid();
                break;
            }
        }
    }
    fn insert(&mut self, label: u64, focal: u64) {
        for l in &mut self.lenses[..self.len] {
            if l.label() == label {
                l.set_focal(focal);
                return;
            }
        }
        if self.len >= MAX_LENSES {
            self.gc();
        }
        self.lenses[self.len] = Lens::new(label, focal);
        self.len += 1;
    }
    // should not happen with a large enough MAX_LENSES
    fn gc(&mut self) {
        let mut new = Self::default();
        for lens in &self.lenses {
            if lens.is_valid() {
                new.lenses[new.len] = *lens;
                new.len += 1;
            }
        }
        if new.len >= MAX_LENSES {
            panic!("failed to gc the box, too many labels are present")
        }
        std::mem::swap(self, &mut new);
    }
}

fn run(input: &str) -> u64 {
    let mut hash = 0;
    let mut label = 0;
    let mut boxes = [Box::default(); 256];
    let mut cur = 0;
    let bytes = input.as_bytes();
    while cur < bytes.len() {
        match bytes[cur] {
            b',' => {
                hash = 0;
                label = 0;
            }
            b'-' => boxes[hash].remove(label),
            b'=' => {
                cur += 1;
                let focal = (bytes[cur] - b'0') as u64;
                boxes[hash].insert(label, focal);
            }
            b'\n' => {}
            b => {
                hash += b as usize;
                hash *= 17;
                hash &= 0xFF;
                label <<= 8;
                label |= b as u64;
            }
        }
        cur += 1;
    }
    let mut res = 0;
    for (i, bx) in boxes.iter().enumerate() {
        res += (i + 1) as u64 * bx.focusing_power();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
