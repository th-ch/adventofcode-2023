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

    pub fn consume_whitespaces(&mut self) {
        while self.curr_char() == Some(b' ') {
            self.offset += 1;
        }
    }

    pub fn consume_while<'b, F: FnMut(u8) -> bool>(&'b mut self, mut f: F) -> &'a str {
        let start = self.offset;
        while self.curr_char().map(&mut f).unwrap_or(false) {
            self.offset += 1;
        }
        &self.data[start..self.offset]
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

    pub fn consume_numeric<'b>(&'b mut self) -> &'a str {
        let start = self.offset;
        while self
            .curr_char()
            .map(|c| c.is_ascii_digit())
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
        let mut res = 0;
        let mut c = *self.data.as_bytes().get(self.offset)?;
        if !c.is_ascii_digit() {
            return None;
        }
        loop {
            if !c.is_ascii_digit() {
                break;
            }
            self.offset += 1;
            res *= 10;
            res += (c - b'0') as u32;
            c = if let Some(&c) = self.data.as_bytes().get(self.offset) {
                c
            } else {
                break;
            }
        }
        Some(res)
    }
}
