use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

struct Part {
    categories: [u16; 4],
}

impl Part {
    fn value(&self) -> usize {
        self.categories.iter().sum::<u16>() as usize
    }
    fn parse(input: &[u8]) -> Self {
        //{x=787,m=2655,a=1222,s=2876}
        let mut categories = [0; 4];
        let mut cur = 3;
        let mut category = 0;
        loop {
            let mut value = 0;

            while let b'0'..=b'9' = input[cur] {
                value *= 10;
                value += (input[cur] - b'0') as u16;
                cur += 1;
            }
            categories[category] = value;
            category += 1;
            if category == 4 {
                return Self { categories };
            }
            while !input[cur].is_ascii_digit() {
                cur += 1
            }
        }
    }
}

type WorkflowsPtr = u16;
const ACCEPTED: WorkflowsPtr = WorkflowsPtr::MAX;
const REJECTED: WorkflowsPtr = WorkflowsPtr::MAX - 1;

struct Rule {
    value: u16,
    category: u8,
    inferior: bool,
    result: WorkflowsPtr,
}

const MAX_CHAR: usize = 27;
const MAX_NB_CHAR: u32 = 3;
const MAX_WORKFLOWS: usize = MAX_CHAR.pow(MAX_NB_CHAR);

struct WorkflowsList([Option<Vec<Rule>>; MAX_WORKFLOWS]);
const DEFAULT: Option<Vec<Rule>> = None;

impl WorkflowsList {
    const fn new() -> Self {
        Self([DEFAULT; MAX_WORKFLOWS])
    }
    fn parse_workflow(&mut self, input: &[u8]) {
        // px{a<2006:qkq,m>2090:A,rfg}
        let mut rules = Vec::new();
        let mut cur = 0;
        while input[cur] != b'{' {
            cur += 1;
        }
        let name = name2id(&input[0..cur]);
        cur += 1;
        let mut stop = false;
        while !stop {
            let mut inferior = true;
            let mut value = 0;
            let mut category = 0;
            // detect if on the last rule
            if input[cur + 1] == b'<' || input[cur + 1] == b'>' {
                category = match input[cur] {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => panic!(),
                };
                cur += 1;
                inferior = input[cur] == b'<';
                loop {
                    cur += 1;
                    match input[cur] {
                        b'0'..=b'9' => {
                            value *= 10;
                            value += (input[cur] - b'0') as u16;
                        }
                        _ => break,
                    }
                }
                cur += 1;
            } else {
                stop = true;
            }
            let res_start = cur;
            while input[cur].is_ascii_alphabetic() {
                cur += 1;
            }
            let result = name2id(&input[res_start..cur]);
            rules.push(Rule {
                value,
                inferior,
                category,
                result,
            });
            cur += 1;
        }
        self.0[name as usize] = Some(rules);
    }

    fn run(&self, part: Part) -> usize {
        let mut rulesptr = name2id(b"in");
        'outer: loop {
            if rulesptr == ACCEPTED {
                return part.value();
            } else if rulesptr == REJECTED {
                return 0;
            }
            let rules = self.0[rulesptr as usize].as_ref().expect("Rule not found!");
            for r in &rules[..rules.len() - 1] {
                if r.inferior {
                    if part.categories[r.category as usize] < r.value {
                        rulesptr = r.result;
                        continue 'outer;
                    }
                } else if part.categories[r.category as usize] > r.value {
                    rulesptr = r.result;
                    continue 'outer;
                }
            }
            rulesptr = rules.last().unwrap().result
        }
    }
}

fn name2id(name: &[u8]) -> WorkflowsPtr {
    if name == b"A" {
        ACCEPTED
    } else if name == b"R" {
        REJECTED
    } else {
        assert!(name.len() <= 3);
        let mut res = 0;
        let mut cur = 0;
        while cur < name.len() {
            res *= MAX_CHAR as u16;
            res += (name[cur] - b'a') as WorkflowsPtr + 1;
            cur += 1;
        }
        res
    }
}

fn run(input: &str) -> usize {
    let mut workflows = WorkflowsList::new();
    let mut lines = input.lines();
    let mut res = 0;
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        workflows.parse_workflow(l.as_bytes());
    }
    for l in lines {
        if l.is_empty() {
            break;
        }
        let part = Part::parse(l.as_bytes());
        res += workflows.run(part);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"),
            19114
        )
    }
}
