use std::{collections::VecDeque, vec};

type StateIndex = u8;

#[derive(Debug, Clone, Copy)]
struct OptionIndex(StateIndex);

// Fold None in max value of index type
impl OptionIndex {
    fn some(i: usize) -> Self {
        Self(i as StateIndex)
    }
    fn none() -> Self {
        Self(StateIndex::MAX)
    }

    fn get(self) -> Option<usize> {
        if self.0 == StateIndex::MAX {
            None
        } else {
            Some(self.0 as usize)
        }
    }
}

#[derive(Debug)]
struct Match {
    entry: StateIndex,
    previous_match: OptionIndex,
}

#[derive(Debug)]
struct TransitionTables {
    table: Vec<OptionIndex>,
    row_len: usize,
}

impl TransitionTables {
    fn new(row_len: usize) -> Self {
        Self {
            table: Vec::new(),
            row_len,
        }
    }
    fn get(&self, state_idx: usize, class: u8) -> OptionIndex {
        self.table[state_idx * self.row_len + class as usize]
    }

    fn number_of_states(&self) -> usize {
        self.table.len() / self.row_len
    }

    fn push_state(&mut self) {
        self.table
            .extend((0..self.row_len).map(|_| OptionIndex::none()));
    }

    fn get_mut(&mut self, state_idx: usize, class: u8) -> &mut OptionIndex {
        &mut self.table[state_idx * self.row_len + class as usize]
    }

    fn get_state_2_mut(&mut self, state_idxs: [usize; 2]) -> [&mut [OptionIndex]; 2] {
        if state_idxs[0] == state_idxs[1] {
            panic!()
        }
        let idxs = [state_idxs[0] * self.row_len, state_idxs[1] * self.row_len];
        let slice = self.table.as_mut_ptr();
        unsafe {
            [
                std::slice::from_raw_parts_mut(slice.add(idxs[0]), self.row_len),
                std::slice::from_raw_parts_mut(slice.add(idxs[1]), self.row_len),
            ]
        }
    }

    fn get_state(&self, state_idx: usize) -> &[OptionIndex] {
        &self.table[state_idx * self.row_len..(state_idx + 1) * self.row_len]
    }
    fn get_state_mut(&mut self, state_idx: usize) -> &mut [OptionIndex] {
        &mut self.table[state_idx * self.row_len..(state_idx + 1) * self.row_len]
    }
}

#[derive(Debug)]
pub struct AhoCorasik {
    state_transitions: TransitionTables,
    state_entries: Vec<OptionIndex>,
    state_classes: Vec<u8>,
    state_matches: Vec<OptionIndex>,
    matches: Vec<Match>,
    entries: Vec<String>,
    byte_classes: [u8; 256],
}

fn compute_byte_class(patterns: &[String]) -> ([u8; 256], usize) {
    let mut bset: [u64; 4] = [0; 4];
    for p in patterns {
        for c in p.as_bytes() {
            bset[(c >> 6) as usize] |= 1 << (c & 0b111111);
        }
    }
    let mut byte_classes = [0; 256];
    let mut class = 0;
    for c in 0_u8..=255 {
        if bset[(c >> 6) as usize] & (1 << (c & 0b111111)) != 0 {
            class += 1;
            byte_classes[c as usize] = class;
        }
    }
    (byte_classes, class as usize + 1)
}

impl AhoCorasik {
    pub fn new(patterns: Vec<String>) -> Self {
        let (byte_classes, nb_of_classes) = compute_byte_class(&patterns);
        let mut state_transitions = TransitionTables::new(nb_of_classes);
        state_transitions.push_state();
        let mut s = Self {
            state_classes: vec![0],
            state_entries: vec![OptionIndex::none()],
            state_matches: vec![OptionIndex::none()],
            matches: vec![],
            state_transitions: state_transitions,
            entries: Vec::new(),
            byte_classes,
        };
        for pat in patterns {
            s.insert_trie(pat);
        }
        s.build_failure_transitions();
        s
    }

    fn insert_trie(&mut self, s: String) {
        let mut v = 0;
        for &c in s.as_bytes() {
            let c: u8 = self.byte_classes[c as usize];
            let n = match self.state_transitions.get(v, c).get() {
                Some(n) => n,
                None => {
                    let n = self.state_transitions.number_of_states();
                    self.state_transitions.push_state();
                    self.state_classes.push(c);
                    self.state_entries.push(OptionIndex::none());
                    self.state_matches.push(OptionIndex::none());
                    *self.state_transitions.get_mut(v, c) = OptionIndex::some(n);
                    n
                }
            };
            v = n
        }
        self.state_entries[v] = OptionIndex::some(self.entries.len());
        self.entries.push(s);
    }

    fn build_failure_transitions(&mut self) {
        let mut longest_suffixes = vec![0; self.state_transitions.number_of_states()];
        let mut bft_nodes = VecDeque::new();
        for n in self.state_transitions.get_state(0) {
            if let Some(i) = n.get() {
                bft_nodes.push_back((0_usize, i));
            }
        }
        for t in self.state_transitions.get_state_mut(0) {
            if t.get().is_none() {
                *t = OptionIndex::some(0);
            }
        }

        loop {
            let Some((parent_state, v)) = bft_nodes.pop_front() else {
                break;
            };
            // push children for bft
            for n in self.state_transitions.get_state(v) {
                if let Some(i) = n.get() {
                    bft_nodes.push_back((v, i));
                }
            }

            // Traverse suffixes of parent to find one that has a transition on the current node character
            let c = self.state_classes[v];

            let longest_suffix = if parent_state == 0 {
                0
            } else {
                self.state_transitions
                    .get(longest_suffixes[parent_state], c)
                    .get()
                    .unwrap()
            };
            longest_suffixes[v] = longest_suffix;

            // build matches
            self.state_matches[v] = if let Some(entry) = self.state_entries[v].get() {
                let match_idx = self.matches.len();
                self.matches.push(Match {
                    entry: entry as StateIndex,
                    previous_match: self.state_matches[longest_suffix],
                });
                OptionIndex::some(match_idx)
            } else {
                self.state_matches[longest_suffix]
            };

            // Copy failure state transitions
            let [suffix, v] = self.state_transitions.get_state_2_mut([longest_suffix, v]);
            for (tv, tl) in v.iter_mut().zip(suffix.iter()) {
                if tv.get().is_none() {
                    *tv = *tl
                }
            }
        }
    }

    pub fn search<'a: 'b, 'b>(
        &'a self,
        haystack: &'b [u8],
    ) -> impl Iterator<Item = (usize, &'a str)> + 'b {
        let mut current_state = 0;
        let mut idx = 0;
        std::iter::from_fn(move || -> Option<(usize, OptionIndex)> {
            loop {
                if idx >= haystack.len() {
                    return None;
                }
                let c = self.byte_classes[haystack[idx] as usize];

                current_state = self.state_transitions.get(current_state, c).get().unwrap();
                idx += 1;
                if let Some(m) = self.state_matches[current_state].get() {
                    return Some((idx, OptionIndex::some(m)));
                }
            }
        })
        .flat_map(move |(idx, state_match)| {
            let mut match_idx = state_match;
            std::iter::from_fn(move || {
                let m = &self.matches[match_idx.get()?];
                match_idx = m.previous_match;
                let entry = self.entries[m.entry as usize].as_str();
                Some((idx - entry.len(), entry))
            })
        })
    }
}

#[test]
fn test_aho_corasick() {
    let searcher = AhoCorasik::new(vec![
        "a".to_owned(),
        "ab".to_owned(),
        "bc".to_owned(),
        "bca".to_owned(),
        "c".to_owned(),
        "caa".to_owned(),
    ]);

    let matches = searcher.search(b"bcaab").collect::<Vec<_>>();
    assert_eq!(
        matches,
        vec![
            (0, "bc"),
            (1, "c"),
            (0, "bca"),
            (2, "a"),
            (1, "caa"),
            (3, "a"),
            (3, "ab")
        ]
    );
}
