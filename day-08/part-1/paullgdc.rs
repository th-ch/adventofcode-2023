use std::collections::HashMap;
use std::env::args;
use std::time::Instant;

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

type StateId = u16;

const START_STATE: StateId = 0;
const END_STATE: StateId = 1;

#[derive(Debug, Clone)]
struct State {
    transitions: [StateId; 2],
}

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);

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
    let mut state_ids: HashMap<&str, u16> = HashMap::new();
    state_ids.insert("AAA", START_STATE);
    state_ids.insert("ZZZ", END_STATE);
    let mut state_transistions: Vec<Option<State>> = vec![None; 2];
    while t.curr_char().is_some() {
        let state_name = t.consume_name();
        t.consume_fixed(" = (").unwrap();
        let left = t.consume_name();
        t.consume_fixed(", ").unwrap();
        let right = t.consume_name();
        t.consume_fixed(")").unwrap();
        t.advance(1);

        let state = get_state_id(&mut state_ids, &mut state_transistions, state_name);
        let left = get_state_id(&mut state_ids, &mut state_transistions, left);
        let right = get_state_id(&mut state_ids, &mut state_transistions, right);
        state_transistions[state as usize] = Some(State {
            transitions: [left, right],
        });
    }

    let states: Vec<State> = state_transistions
        .into_iter()
        .collect::<Option<_>>()
        .unwrap();

    let mut current_state = START_STATE;
    let mut seq_idx = 0;
    let mut steps = 0;
    assert!(sequence.len() > 0);
    while current_state != END_STATE {
        current_state = states[current_state as usize].transitions[sequence[seq_idx] as usize];
        seq_idx = (seq_idx + 1) % sequence.len();
        steps += 1;
    }

    steps
}

fn get_state_id<'a, 'b: 'a>(
    name_to_id: &'a mut HashMap<&'b str, StateId>,
    states: &mut Vec<Option<State>>,
    state_name: &'b str,
) -> StateId {
    let next_entry = name_to_id.len() as StateId;
    let state = *name_to_id.entry(state_name).or_insert(next_entry);
    if state as usize >= states.len() {
        states.push(None);
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
}
