use enum_iterator::{all, Sequence};
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, rc::Rc};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Sequence)]
pub enum Op {
    Add,
    Sub,
    InvSub,
    Mul,
    Div,
    InvDiv,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct AppliedOp(Op, u32, u32, u32);

impl Display for AppliedOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Op::Add => write!(f, "{} + {} = {}", self.1, self.2, self.3),
            Op::Sub => write!(f, "{} - {} = {}", self.1, self.2, self.3),
            Op::InvSub => write!(f, "{} - {} = {}", self.2, self.1, self.3),
            Op::Mul => write!(f, "{} * {} = {}", self.1, self.2, self.3),
            Op::Div => write!(f, "{} / {} = {}", self.1, self.2, self.3),
            Op::InvDiv => write!(f, "{} / {} = {}", self.2, self.1, self.3),
        }
    }
}

#[derive(Hash, Eq, Debug)]
pub struct State {
    pub numbers: Vec<u32>,
    pub previous_state: Option<Rc<State>>,
    pub previous_op: Option<AppliedOp>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.numbers == other.numbers
    }
}

impl Op {
    fn apply(&self, term_a: u32, term_b: u32) -> Option<u32> {
        match self {
            Op::Add => Some(term_a + term_b),
            Op::Sub => {
                if term_a > term_b {
                    Some(term_a - term_b)
                } else {
                    None
                }
            }
            Op::InvSub => {
                if term_b > term_a {
                    Some(term_b - term_a)
                } else {
                    None
                }
            }
            Op::Mul => Some(term_a * term_b),
            Op::Div => {
                if term_a % term_b == 0 {
                    Some(term_a / term_b)
                } else {
                    None
                }
            }
            Op::InvDiv => {
                if term_b % term_a == 0 {
                    Some(term_b / term_a)
                } else {
                    None
                }
            }
        }
    }
}

pub fn solve(target: u32, numbers: &[u32]) -> Option<Rc<State>> {
    let target = target;

    let start = Rc::new(State {
        numbers: Vec::from(numbers),
        previous_state: None,
        previous_op: None,
    });

    let mut discovery = vec![start];
    let mut visited = HashSet::<Rc<State>>::new();

    let mut result = None;

    while let Some(state) = discovery.pop() {
        if !visited.contains(&state) {
            if state.numbers.contains(&target) {
                result = Some(Rc::clone(&state));
                break;
            }

            let numbers = state.numbers.clone();

            for mut pair in state.numbers.iter().enumerate().combinations(2) {
                let (Some(term_a), Some(term_b) )= (pair.pop(), pair.pop() )else {
                    continue;
                };

                let try_op = |op: Op| {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(term_a.0);
                    new_numbers.remove(term_b.0);

                    let result = op.apply(*term_a.1, *term_b.1);

                    if let Some(result) = result {
                        new_numbers.push(result);

                        let new_state = Rc::new(State {
                            numbers: new_numbers,
                            previous_state: Some(Rc::clone(&state)),
                            previous_op: Some(AppliedOp(op, *term_a.1, *term_b.1, result)),
                        });

                        discovery.push(new_state);
                    }
                };

                all::<Op>().for_each(try_op);
            }
        }

        visited.insert(Rc::clone(&state));
    }

    result
}
