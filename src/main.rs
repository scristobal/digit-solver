use clap::Parser;
use std::{collections::HashSet, rc::Rc, time::Instant};

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target number
    #[arg(short, long)]
    target: u32,

    /// Numbers to combine
    #[arg(short, long, num_args = 6)]
    numbers: Vec<u32>,
}

#[derive(Hash, Eq, Debug)]
struct State {
    numbers: Vec<u32>,
    previous_state: Option<Rc<State>>,
    previous_op: Option<(u32, Op, u32, u32)>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.numbers == other.numbers
    }
}

fn main() {
    let args = Args::parse();

    let target = args.target;

    let start = Rc::new(State {
        numbers: args.numbers,
        previous_state: None,
        previous_op: None,
    });

    let start_time = Instant::now();

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

    if result.is_some() {
        let mut operations = Vec::<(u32, Op, u32, u32)>::new();

        while let Some(ref state) = result {
            if let Some(op) = &state.previous_op {
                operations.push(*op)
            }

            result = state.previous_state.clone()
        }

        let duration = start_time.elapsed();

        println!(
            "Found solution in {:?} after {} operations",
            duration,
            visited.len()
        );

        operations.iter().rev().for_each(|op| println!("{:?}", op));
    } else {
        println!("no solution found")
    }
}
