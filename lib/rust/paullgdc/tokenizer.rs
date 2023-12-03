use std::fmt::Debug;

pub struct Tokenizer<'a> {
    data: &'a str,
    offset: usize,
}

impl Debug for Tokenizer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data[self.offset..], f)
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data, offset: 0 }
    }

    pub fn advance(&mut self, n: usize) {
        self.offset += n;
    }

    pub fn consume_fixed(&mut self, s: &str) -> Option<()> {
        if self.data[self.offset..].starts_with(s) {
            self.offset += s.len();
            Some(())
        } else {
            None
        }
    }

    pub fn consume_name<'b>(&'b mut self) -> &'a str {
        let start = self.offset;
        while self
            .curr_char()
            .map(|c| c.is_ascii_alphabetic())
            .unwrap_or(false)
        {
            self.offset += 1;
        }
        &self.data[start..self.offset]
    }

    pub fn consume_until(&mut self, until_c: u8) {
        while self.curr_char().map(|c| c != until_c).unwrap_or(false) {
            self.offset += 1;
        }
    }

    pub fn curr_char(&mut self) -> Option<u8> {
        self.data.as_bytes().get(self.offset).copied()
    }

    pub fn consume_u32(&mut self) -> Option<u32> {
        let start = self.offset;
        while self
            .data
            .as_bytes()
            .get(self.offset)
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            self.offset += 1;
        }
        self.data[start..self.offset].parse().ok()
    }
}
