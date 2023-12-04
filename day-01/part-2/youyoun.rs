use std::collections::HashMap;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
    number: u32,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
            number: 0,
        }
    }

    fn insert(&mut self, s: &str, number: u32) {
        let mut node: &mut Node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_insert(Node::new());
        }
        node.is_end = true;
        node.number = number;
    }

    fn find_first_match(&self, s: &str) -> Option<u32> {
        let mut current_node: &Node = self;

        for c in s.chars() {
            if let Some(child) = current_node.children.get(&c) {
                current_node = child;
            } else {
                break;
            }
        }
        if current_node.is_end {
            Some(current_node.number)
        } else {
            None
        }
    }
}

fn run(input: &str) -> u32 {
    // Your code goes here
    let mut sum_: u32 = 0;

    let mut number_search_tree: Node = Node::new();

    let word_numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let numbers: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for i in 0..9 {
        number_search_tree.insert(word_numbers[i], numbers[i].to_digit(10).unwrap());
        number_search_tree.insert(
            numbers[i].to_string().as_str(),
            numbers[i].to_digit(10).unwrap(),
        );
    }

    for line in input.lines() {
        let mut is_first_set: bool = false;
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for i in 0..line.len() {
            let n_ = number_search_tree.find_first_match(&line[i..]);
            if n_.is_some() {
                if !is_first_set {
                    first = n_.unwrap();
                    is_first_set = true;
                }
                last = n_.unwrap();
            }
        }
        sum_ += first * 10 + last;
    }
    sum_
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }
}
