use std::time::Instant;

use clap::Parser;
use digits_solver::solve;

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

fn main() {
    let args = Args::parse();

    let target = args.target;
    let numbers = args.numbers;

    let start_time = Instant::now();

    let mut result = solve(target, &numbers);

    let duration = start_time.elapsed();

    println!("Finished in {:?}", duration);

    let mut operations = vec![];

    while let Some(state) = result {
        if let Some(op) = &state.previous_op {
            operations.push(*op)
        }

        result = state.previous_state.clone()
    }

    if operations.len() > 0 {
        println!("Solution found:");
        operations.iter().rev().for_each(|op| println!("{}", op));
    } else {
        println!("No solution found")
    }
}
