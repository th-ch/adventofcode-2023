use std::fmt::Debug;

pub struct Grid<T> {
    store: Vec<T>,
    row_len: usize,
    column_len: usize,
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut l = f.debug_list();
        for j in 0..self.dims().1 {
            l.entry(&&self.store[j * self.row_len..(j + 1) * self.row_len]);
        }
        l.finish()
    }
}

impl<T> Grid<T> {
    pub fn new_from_vec(dims: (usize, usize), v: Vec<T>) -> Self {
        assert!(dims.0 * dims.1 == v.len());
        Self {
            store: v,
            row_len: dims.0,
            column_len: dims.1,
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.row_len, self.column_len)
    }

    pub fn get(&self, (i, j): (usize, usize)) -> Option<&T> {
        if i >= self.row_len || j >= self.column_len {
            None
        } else {
            Some(unsafe { self.store.get_unchecked(i + j * self.row_len) })
        }
    }

    pub fn get_mut(&mut self, (i, j): (usize, usize)) -> Option<&mut T> {
        if i >= self.row_len || j >= self.column_len {
            None
        } else {
            Some(unsafe { self.store.get_unchecked_mut(i + j * self.row_len) })
        }
    }
}

impl Grid<u8> {
    pub fn from_input(input: &str) -> Self {
        let mut store = Vec::new();
        let mut row_len = 0;
        for l in input.as_bytes().split(|&b| b == b'\n') {
            if l.is_empty() {
                continue;
            }
            row_len = l.len();
            store.extend(l);
        }
        let column_len = store.len() / row_len;
        Self {
            store,
            row_len,
            column_len,
        }
    }
}
