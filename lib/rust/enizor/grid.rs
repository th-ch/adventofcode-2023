use std::ops::Index;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left = 0b1,
    #[default]
    Right = 0b10,
    Up = 0b100,
    Down = 0b1000,
}

use Direction::*;
pub const ALL_DIRECTIONS: [Direction; 4] = [Left, Right, Up, Down];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// A grid referencing a str, with newlines \n separating lines
#[derive(Clone, Copy)]
pub struct StrGrid<'a> {
    pub width: usize,
    pub height: usize,
    pub data: &'a [u8],
}

impl<'a> StrGrid<'a> {
    pub fn from_input(input_str: &'a str) -> StrGrid {
        let bytes = input_str.as_bytes();
        let w = bytes
            .iter()
            .position(|b| *b == b'\n')
            .expect("No line ending!")
            + 1;
        // leeway for some additional useless chars beyond the last line
        // such as a trailing newline
        let h = (bytes.len() + 1) / w;
        Self {
            width: w,
            height: h,
            data: &bytes[..h * w - 1],
        }
    }
    /// Constructs a Grid referencing the input string.
    /// The lines must be separated by \n
    #[inline(always)]
    pub const fn cur(&self, pos: Position) -> usize {
        pos.x + self.width * pos.y
    }

    #[inline(always)]
    pub const fn from_cur(&self, idx: usize) -> Position {
        Position {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    #[inline(always)]
    pub fn step(&self, pos: Position, dir: Direction) -> Option<Position> {
        let mut pos2 = pos;
        if self.step_mut(&mut pos2, dir) {
            Some(pos)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn step_mut(&self, pos: &mut Position, dir: Direction) -> bool {
        match (dir, pos.x, pos.y) {
            (Left, 0, _) => return false,
            (Right, _, _) if pos.x == self.width - 2 => return false,
            (Up, _, 0) => return false,
            (Down, _, _) if pos.y == self.height - 1 => return false,
            (Left, _, _) => pos.x -= 1,
            (Right, _, _) => pos.x += 1,
            (Up, _, _) => pos.y -= 1,
            (Down, _, _) => pos.y += 1,
        }
        true
    }
}

impl<'a> Index<Position> for StrGrid<'a> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, pos: Position) -> &Self::Output {
        &self.data[self.cur(pos)]
    }
}

use std::fmt;
impl<'a> fmt::Debug for StrGrid<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StrGrid  {{ width: {}, height: {}, data:\n{}\n}}",
            self.width,
            self.height,
            &super::utils::debug_ascii(self.data)
        )
    }
}
