use std::collections::VecDeque;

use super::bitset::{bitset_size, ArrayBitSet};

type ModuleId = u8;
const MAX_MODULES: usize = 64;
const INVALID_ID: ModuleId = ModuleId::MAX;
const BROADCASTER_ID: ModuleId = 0;
const RX_ID: ModuleId = BROADCASTER_ID + 1;
const ALPHABET_SIZE: usize = 27; // alphabet+empty char

struct Name2Id {
    hash_to_id: Vec<ModuleId>,
    max_id: ModuleId,
}

fn binary_gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    // maximum power of 2 divNameing both u & v
    let max_power_2 = (u | v).trailing_zeros();

    // Turn both to their odd parts
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    // Use the simple gcd(u, v) = gcd(|u − v|, min(u, v)), for the odd parts of u and v.
    // as u-v is even but min(u, v) is odd, we can continue to strip it off the even part
    while u != v {
        if u < v {
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // Multiply back the even part
    u << max_power_2
}

fn lcm(u: usize, v: usize) -> usize {
    u * v / binary_gcd(u, v)
}

impl Name2Id {
    fn init() -> Self {
        let mut name2id = Name2Id {
            hash_to_id: vec![INVALID_ID; 1024],
            max_id: 0,
        };
        let broadcast = name2id.insert(Self::hash(b"broadcaster"));
        assert_eq!(broadcast, BROADCASTER_ID);
        let rx = name2id.insert(Self::hash(b"rx"));
        assert_eq!(rx, RX_ID);
        name2id
    }
    #[inline(always)]
    fn hash(name: &[u8]) -> usize {
        if name == b"broadcaster" {
            0
        } else {
            assert!(!name.is_empty());
            let mut res = 0;
            let mut cur = 0;
            while cur < name.len() {
                res *= ALPHABET_SIZE;
                res += (name[cur] - b'a' + 1) as usize;
                cur += 1;
            }
            res
        }
    }
    #[inline(always)]
    fn insert(&mut self, hash: usize) -> ModuleId {
        assert!((self.max_id as usize) < MAX_MODULES);
        self.hash_to_id[hash] = self.max_id;
        self.max_id += 1;
        self.max_id - 1
    }
    #[inline(always)]
    fn get_or_insert(&mut self, name: &[u8]) -> ModuleId {
        let hash = Self::hash(name);
        if hash > self.hash_to_id.len() {
            self.hash_to_id.resize(hash * 2, INVALID_ID);
            self.insert(hash)
        } else if self.hash_to_id[hash] != INVALID_ID {
            self.hash_to_id[hash]
        } else {
            self.insert(hash)
        }
    }
}

type GlobalState = ArrayBitSet<{ bitset_size(MAX_MODULES) }>;

enum ModuleType {
    FlipFlop(bool),
    Conjunction(GlobalState),
    Broadcaster,
}
use ModuleType::*;

struct Connections([ModuleId; MAX_MODULES]);
const NO_CONNECTIONS: Connections = Connections([INVALID_ID; MAX_MODULES]);

impl Connections {
    #[inline]
    fn iter(&self) -> impl Iterator<Item = &ModuleId> {
        self.0.iter().take_while(|id| **id != INVALID_ID)
    }
    #[inline]
    fn push(&mut self, id: ModuleId) {
        let p = self
            .0
            .iter()
            .position(|id| *id == INVALID_ID)
            .expect("No space left to insert a module!");
        self.0[p] = id;
    }
}

struct Module {
    outputs: Connections,
    module_type: ModuleType,
    inputs: Connections,
}

const INVALID_MODULE: Module = Module {
    outputs: NO_CONNECTIONS,
    inputs: NO_CONNECTIONS,
    module_type: Broadcaster,
};

impl Module {
    #[inline]
    fn parse(input: &str, name2id: &mut Name2Id) -> (Self, ModuleId) {
        let mut module = INVALID_MODULE;
        let bytes = input.as_bytes();
        let id;
        let mut cur = 1;
        while bytes[cur].is_ascii_alphabetic() {
            cur += 1;
        }
        (module.module_type, id) = match bytes[0] {
            b'b' => {
                assert!(bytes.len() > 11);
                assert_eq!(&bytes[..11], b"broadcaster");
                (Broadcaster, BROADCASTER_ID)
            }
            b'%' => (FlipFlop(false), name2id.get_or_insert(&bytes[1..cur])),
            b'&' => (
                Conjunction(GlobalState::new()),
                name2id.get_or_insert(&bytes[1..cur]),
            ),
            b => panic!("invalid module type {:?}", b as char),
        };
        cur += 4;
        while cur < bytes.len() {
            let start = cur;
            while cur < bytes.len() && bytes[cur].is_ascii_alphabetic() {
                cur += 1;
            }
            module
                .outputs
                .push(name2id.get_or_insert(&bytes[start..cur]));
            cur += 2;
        }
        (module, id)
    }
    #[inline]
    fn recv(&mut self, input_id: ModuleId, signal: bool) -> Option<bool> {
        match &mut self.module_type {
            Broadcaster => Some(false),
            FlipFlop(state) => {
                if signal {
                    // If a flip-flop module receives a high pulse, it is ignored and nothing happens
                    None
                } else {
                    // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse
                    *state = !*state;
                    Some(*state)
                }
            }
            Conjunction(state) => {
                // When a pulse is received, the conjunction module first updates its memory for that input
                if signal {
                    state.set(input_id);
                } else {
                    state.reset(input_id);
                }
                // If it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                let mut all_on = true;
                for &i in self.inputs.iter() {
                    all_on &= state.test(i);
                }
                Some(!all_on)
            }
        }
    }
}

pub struct MachineConfiguration {
    modules: [Module; MAX_MODULES],
}

impl MachineConfiguration {
    pub fn parse(input: &str) -> Self {
        let mut modules = [INVALID_MODULE; MAX_MODULES];
        let mut name2id = Name2Id::init();
        for l in input.lines() {
            if !l.is_empty() {
                let (mut module, id) = Module::parse(l, &mut name2id);
                for &out in module.outputs.iter() {
                    modules[out as usize].inputs.push(id);
                }
                std::mem::swap(&mut modules[id as usize].outputs, &mut module.outputs);
                std::mem::swap(
                    &mut modules[id as usize].module_type,
                    &mut module.module_type,
                );
            }
        }
        Self { modules }
    }
    #[inline]
    pub fn press_button(&mut self) -> (usize, usize) {
        let mut low_count = 0;
        let mut high_count = 0;
        let mut signals = VecDeque::with_capacity(MAX_MODULES / 4);
        signals.push_back((false, INVALID_ID, BROADCASTER_ID));
        while let Some((signal, sender, dst)) = signals.pop_front() {
            if signal {
                high_count += 1;
            } else {
                low_count += 1;
            };
            let receiver = &mut self.modules[dst as usize];
            if let Some(out_signal) = receiver.recv(sender, signal) {
                for o in receiver.outputs.iter() {
                    signals.push_back((out_signal, dst, *o));
                }
            }
        }
        (low_count, high_count)
    }

    // A counter is a set of modules: with
    // * a chain of FlipFlops
    // * two conjunctions c1 and c2
    // * the chain start inputs are both conjuntion c1 and broadcast
    // * the flipFlops form a chain, i.e. they have an output to the next flipflop, and an optional one to the conjunction c1
    // * the c1 conjunction outputs are only into the chain and the other conjunction c2
    // * c1 properly resets its memory i.e. its output will reset all flipflops back to 0
    // * c2 is an inverter, (i.e. its only input is c1)
    // A counter will send a High pulse every counter_value button presses
    fn is_counter(&self, start_chain: ModuleId) -> Option<(ModuleId, usize)> {
        // check start
        let mut previous = BROADCASTER_ID;
        let mut counter_value: usize = 0;
        let mut reset_value: usize = 0;
        let mut opt_c1 = None;
        let mut next = Some(start_chain);
        let mut chain_len = 0;
        let mut chain = Vec::new();
        let high_bit = (usize::MAX >> 1) + 1;
        while let Some(ptr) = next {
            chain.push(ptr);
            chain_len += 1;
            // we are reading the lowest bits first
            counter_value >>= 1;
            reset_value >>= 1;
            let module = &self.modules[ptr as usize];
            match module.module_type {
                FlipFlop(_) => {}
                _ => return None,
            }
            for &i in module.inputs.iter() {
                if i != previous {
                    match opt_c1 {
                        Some(c1) if i != c1 => return None,
                        None => opt_c1 = Some(i),
                        _ => {}
                    }
                    // this input is here to reset back the counter;
                    reset_value |= high_bit;
                }
            }
            let c1 = opt_c1?;
            next = None;
            for &i in module.outputs.iter() {
                if i != c1 {
                    // early return if it is already Some
                    next = Some(next.xor(Some(i))?);
                } else {
                    // an output to c1 is a 1 in the binary representation of the counter value
                    counter_value |= high_bit;
                }
            }
            previous = ptr;
        }
        // Check that the rest correctly set all flipflops back to 0
        // all ones up to the chain_len th bit then 0s
        if counter_value + reset_value != 0 {
            return None;
        }

        counter_value >>= (std::mem::size_of::<usize>() * 8) - chain_len;
        // check that c1 only connects to an inverter c2 and the chain
        let c1m = &self.modules[opt_c1? as usize];
        match c1m.module_type {
            Conjunction(_) => {}
            _ => return None,
        }
        for i in c1m.inputs.iter() {
            if !chain.contains(i) {
                return None;
            }
        }
        let mut opt_c2 = None;
        for &i in c1m.outputs.iter() {
            if !chain.contains(&i) {
                // early return if it is already Some i.e. more than one
                opt_c2 = Some(next.xor(Some(i))?);
            }
        }
        // check that c2 only connects c1 as input and a single output c3
        let c2m = &self.modules[opt_c2? as usize];
        match c2m.module_type {
            Conjunction(_) => {}
            _ => return None,
        }

        for &i in c2m.inputs.iter() {
            if i != opt_c1? {
                return None;
            }
        }
        let mut opt_c3 = None;
        for &i in c2m.outputs.iter() {
            // any output outside c3 is not a chain
            opt_c3 = Some(next.xor(Some(i))?);
        }
        Some((opt_c3?, counter_value))
    }

    // Tests if the configuration is N counters that all output to single conjunction c3
    // and c3 is the unique input of rx
    // In this case, rx is started when presses equals the PPCM of all counters
    pub fn is_conjunction_of_counters(&self) -> Option<usize> {
        // find rx and its parent
        let mut opt_c3 = None;
        let broadcast = &self.modules[BROADCASTER_ID as usize];
        let mut combined_len = 1;
        for &start_chain in broadcast.outputs.iter() {
            let (id, cycle) = self.is_counter(start_chain)?;
            match opt_c3 {
                Some(c3) if id != c3 => return None,
                None => opt_c3 = Some(id),
                _ => {}
            }
            combined_len = lcm(combined_len, cycle);
        }
        // broadcast leads to valids chains that lead back to a single c3
        // Check that c3 is a conjunction and only outputs to rx
        let c3m = &self.modules[opt_c3? as usize];
        match c3m.module_type {
            Conjunction(_) => {}
            _ => return None,
        }
        for &i in c3m.outputs.iter() {
            if i != RX_ID {
                return None;
            }
        }
        // check that rx only input from c3 and has no output
        let rxm = &self.modules[RX_ID as usize];
        if let Some(_o) = rxm.outputs.iter().next() {
            return None;
        }
        for &i in rxm.inputs.iter() {
            if i != opt_c3? {
                return None;
            }
        }
        // no need to check c3's input as we checked the outputs of all other nodes reachable from broadcast
        Some(combined_len)
    }
}
