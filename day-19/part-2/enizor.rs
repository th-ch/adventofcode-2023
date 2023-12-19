use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Clone, Copy)]
struct PartConstraint {
    categories: [(u16, u16); 4],
}

impl PartConstraint {
    fn value(&self) -> usize {
        let mut res = 1;
        for (min, max) in self.categories.iter() {
            assert!(max > min);
            res *= (max - min) as usize;
        }
        res
    }
    fn new() -> Self {
        Self {
            categories: [(1, 4001); 4],
        }
    }
    fn and(&mut self, rule: &Rule) -> bool {
        let (min, max) = &mut self.categories[rule.category as usize];
        if rule.inferior {
            if rule.value < *min {
                false
            } else {
                *max = (*max).min(rule.value);
                *max > *min
            }
        } else if rule.value >= *max {
            false
        } else {
            *min = (*min).max(rule.value + 1);
            *max > *min
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

impl Rule {
    fn inverse(&self) -> Rule {
        let inferior = !self.inferior;
        let mut value = self.value;
        if inferior {
            value += 1;
        } else {
            value -= 1;
        }
        Rule {
            value,
            inferior,
            ..*self
        }
    }
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

    fn run(&self) -> usize {
        let start = name2id(b"in");
        let mut stack = vec![(PartConstraint::new(), start)];
        let mut res = 0;
        while let Some((mut part, rulesptr)) = stack.pop() {
            if rulesptr == ACCEPTED {
                res += part.value();
                continue;
            } else if rulesptr == REJECTED {
                continue;
            }
            let rules = self.0[rulesptr as usize].as_ref().expect("Rule not found!");
            for r in &rules[..rules.len() - 1] {
                let mut p1 = part;
                if p1.and(r) {
                    stack.push((p1, r.result));
                }
                if !part.and(&r.inverse()) {
                    continue;
                }
            }
            stack.push((part, rules.last().unwrap().result));
        }
        res
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
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        workflows.parse_workflow(l.as_bytes());
    }
    workflows.run()
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
            167409079868000
        )
    }
}
