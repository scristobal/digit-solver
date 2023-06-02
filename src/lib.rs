use std::{collections::HashSet, rc::Rc};

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Hash, Eq, Debug)]
pub struct State {
    pub numbers: Vec<u32>,
    pub previous_state: Option<Rc<State>>,
    pub previous_op: Option<(u32, Op, u32, u32)>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.numbers == other.numbers
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
                let Some(term_a) = pair.pop() else {
                    continue;
                };

                let Some(term_b) = pair.pop() else {
                    continue;
                };

                let mut new_numbers = numbers.clone();
                new_numbers.remove(term_a.0);
                new_numbers.remove(term_b.0);
                let result = term_a.1 + term_b.1;
                new_numbers.push(result);

                let new_state = Rc::new(State {
                    numbers: new_numbers,
                    previous_state: Some(Rc::clone(&state)),
                    previous_op: Some((*term_a.1, Op::Add, *term_b.1, result)),
                });

                discovery.push(new_state);

                let mut new_numbers = numbers.clone();
                new_numbers.remove(term_a.0);
                new_numbers.remove(term_b.0);
                let result = term_a.1 * term_b.1;
                new_numbers.push(result);

                let new_state = Rc::new(State {
                    numbers: new_numbers,
                    previous_state: Some(Rc::clone(&state)),
                    previous_op: Some((*term_a.1, Op::Mul, *term_b.1, result)),
                });

                discovery.push(new_state);

                if term_a.1 > term_b.1 {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(term_a.0);
                    new_numbers.remove(term_b.0);
                    let result = term_a.1 - term_b.1;
                    new_numbers.push(result);

                    let new_state = Rc::new(State {
                        numbers: new_numbers,
                        previous_state: Some(Rc::clone(&state)),
                        previous_op: Some((*term_a.1, Op::Sub, *term_b.1, result)),
                    });

                    discovery.push(new_state);
                }

                if term_b.1 > term_a.1 {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(term_a.0);
                    new_numbers.remove(term_b.0);
                    let result = term_b.1 - term_a.1;
                    new_numbers.push(result);

                    let new_state = Rc::new(State {
                        numbers: new_numbers,
                        previous_state: Some(Rc::clone(&state)),
                        previous_op: Some((*term_b.1, Op::Sub, *term_a.1, result)),
                    });

                    discovery.push(new_state);
                }

                if term_b.1 % term_a.1 == 0 {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(term_a.0);
                    new_numbers.remove(term_b.0);
                    let result = term_b.1 / term_a.1;
                    new_numbers.push(result);

                    let new_state = Rc::new(State {
                        numbers: new_numbers,
                        previous_state: Some(Rc::clone(&state)),
                        previous_op: Some((*term_b.1, Op::Div, *term_a.1, result)),
                    });

                    discovery.push(new_state);
                }

                if term_a.1 % term_b.1 == 0 {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(term_a.0);
                    new_numbers.remove(term_b.0);
                    let result = term_a.1 / term_b.1;
                    new_numbers.push(result);

                    let new_state = Rc::new(State {
                        numbers: new_numbers,
                        previous_state: Some(Rc::clone(&state)),
                        previous_op: Some((*term_a.1, Op::Div, *term_b.1, result)),
                    });

                    discovery.push(new_state);
                }
            }
        }

        visited.insert(Rc::clone(&state));
    }

    result
}
