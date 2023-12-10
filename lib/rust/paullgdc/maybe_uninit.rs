use std::mem::{ManuallyDrop, MaybeUninit};

use super::bitset::Bitset;

#[derive(Debug)]
pub struct MaybeUninitVec<T> {
    values: std::vec::Vec<MaybeUninit<T>>,
    init: Bitset,
}

impl<T> Default for MaybeUninitVec<T> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            init: Bitset::default(),
        }
    }
}

impl<T> MaybeUninitVec<T> {
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn extend_uninit(&mut self) {
        self.values.push(MaybeUninit::uninit());
        self.init.push(false);
    }

    pub fn set(&mut self, i: usize, value: T) -> Option<()> {
        let is_init = self.init.get(i)?;
        unsafe {
            if is_init {
                self.values.get_unchecked_mut(i).assume_init_drop();
            }
            self.values.get_unchecked_mut(i).write(value);
            self.init.set(i, true);
        }
        Some(())
    }

    pub fn get_all_inits(self) -> Option<std::vec::Vec<T>> {
        if self.init.all() {
            let mut v = ManuallyDrop::new(self.values);
            unsafe {
                Some(std::vec::Vec::from_raw_parts(
                    v.as_mut_ptr() as *mut T,
                    v.len(),
                    v.capacity(),
                ))
            }
        } else {
            None
        }
    }
}
