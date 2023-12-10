use std::collections::HashMap;
use std::env::args;
use std::time::Instant;

use aoc::paullgdc::maybe_uninit::MaybeUninitVec;
use aoc::paullgdc::tokenizer::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const LEFT: u8 = 0;
const RIGHT: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct StateId(u16);

impl StateId {
    const fn new(name: &str, id: usize) -> Self {
        if name.as_bytes()[name.len() - 1] == b'Z' {
            Self(id as u16 | GHOST_END_STATE_MASK)
        } else if name.as_bytes()[name.len() - 1] == b'A' {
            Self(id as u16 | GHOST_START_STATE_MASK)
        } else {
            Self(id as u16)
        }
    }

    fn is_ghost_start(&self) -> bool {
        self.0 & GHOST_START_STATE_MASK != 0
    }

    fn is_ghost_end(&self) -> bool {
        self.0 & GHOST_END_STATE_MASK != 0
    }

    fn as_idx(&self) -> usize {
        (self.0 & !(GHOST_START_STATE_MASK | GHOST_END_STATE_MASK)) as usize
    }
}

const GHOST_START_STATE_MASK: u16 = 1 << 15;
const GHOST_END_STATE_MASK: u16 = 1 << 14;

#[derive(Debug, Clone)]
struct State {
    transitions: [StateId; 2],
}

fn parse<'a>(t: &mut Tokenizer<'a>) -> (Vec<u8>, Vec<State>, Vec<StateId>) {
    let sequence: Vec<_> = t
        .consume_name()
        .as_bytes()
        .iter()
        .map(|d| match d {
            b'L' => LEFT,
            b'R' => RIGHT,
            _ => panic!(),
        })
        .collect();
    t.consume_fixed("\n\n").unwrap();
    let mut state_name_to_id: HashMap<&str, StateId> = HashMap::new();
    let mut state_transistions = MaybeUninitVec::default();
    let mut state_ids = Vec::new();
    while t.curr_char().is_some() {
        let state_name = t.consume_while(|c| c.is_ascii_alphanumeric());
        t.consume_fixed(" = (").unwrap();
        let left = t.consume_while(|c| c.is_ascii_alphanumeric());
        t.consume_fixed(", ").unwrap();
        let right = t.consume_while(|c| c.is_ascii_alphanumeric());
        t.consume_fixed(")").unwrap();
        t.advance(1);

        let state = get_state_id(&mut state_name_to_id, &mut state_transistions, state_name);
        let left = get_state_id(&mut state_name_to_id, &mut state_transistions, left);
        let right = get_state_id(&mut state_name_to_id, &mut state_transistions, right);
        state_transistions.set(
            state.as_idx(),
            State {
                transitions: [left, right],
            },
        );
        state_ids.push(state);
    }
    let states: Vec<State> = state_transistions.get_all_inits().unwrap();
    (sequence, states, state_ids)
}

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);

    let (sequence, states, state_ids) = parse(&mut t);

    (state_ids
        .iter()
        .filter(|s| s.is_ghost_start())
        .map(|&state_id| steps_to_state(state_id, &states, &sequence, |s| s.is_ghost_end()))
        .reduce(|a, b| a * b)
        .unwrap()
        * sequence.len()) as isize
}

fn steps_to_state<F: FnMut(StateId) -> bool>(
    start: StateId,
    states: &[State],
    sequence: &[u8],
    mut is_terminal: F,
) -> usize {
    let mut current_state = start;
    let mut run = 0;
    loop {
        for &d in sequence {
            current_state = states[current_state.as_idx()].transitions[d as usize];
        }
        run += 1;
        if is_terminal(current_state) {
            return run;
        }
    }
}

fn get_state_id<'a, 'b: 'a>(
    name_to_id: &'a mut HashMap<&'b str, StateId>,
    states: &mut MaybeUninitVec<State>,
    state_name: &'b str,
) -> StateId {
    let next_entry = name_to_id.len();
    let state = *name_to_id
        .entry(state_name)
        .or_insert(StateId::new(state_name, next_entry));
    if state.as_idx() >= states.len() {
        states.extend_uninit();
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"),
            2
        )
    }

    #[test]
    fn run_test_2() {
        assert_eq!(
            run("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"),
            6
        )
    }

    #[test]
    fn run_test_3() {
        assert_eq!(
            run("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"),
            6
        )
    }
}
