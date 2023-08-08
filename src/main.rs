use rand::{rngs::SmallRng, SeedableRng};
use std::env;

mod rulesets;
mod tm;
mod turing_machine_memory;

use crate::rulesets::RuleSets;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut symbols = 2;
    let mut states = 4;
    let mut move_instructions = 4;
    let mut steps = 10;
    let mut blindsteps = 0;
    let mut seed: u64 = 1;

    if args.len() > 1 {
        seed = args[1].parse().unwrap_or_default();
        println!("seed = {}", seed);
    }
    if args.len() > 2 {
        symbols = args[2].parse().unwrap_or(2);
        if symbols < 2 {
            symbols = 2
        };
    }
    if args.len() > 3 {
        states = args[3].parse().unwrap_or(2);
        if states < 3 {
            states = 3
        };
    }
    if args.len() > 4 {
        steps = args[4].parse().unwrap_or(10);
        if steps < 0 {
            steps = 0
        };
    }
    if args.len() > 5 {
        blindsteps = args[5].parse().unwrap_or(10);
        if blindsteps < 0 {
            blindsteps = 0
        };
    }

    let mut rng = SmallRng::seed_from_u64(seed);
    let mut rules = RuleSets::new(symbols, states, move_instructions);
    // println!("possible rulesets = {:+.2e}", rules.possible_rule_sets());
    rules.random_ruleset(&mut rng);
    print!("{}", rules);
    println!(
        "with {} symbols, {} states and {} moving-directions {} rules are possible",
        symbols,
        states,
        move_instructions,
        &rules.possible_rules()
    );

    println!("{} possible input combinations", symbols * states);

    // let mut tm = tm::TM::<turing_machine_memory::Memory1D<4>>::new();
    let mut tm = tm::TM::<turing_machine_memory::Memory2D<8, 8>>::new();
    // let mut tm = tm::TM::<turing_machine_memory::Memory3D<15, 7, 3>>::new();
    tm.insert_rules(&rules.rules);
    for _ in 0..blindsteps {
        // println!("{tm}");
        tm.step();
    }

    rules.mutate_ruleset(1, &mut rng);
    tm.insert_rules(&rules.rules);
    print!("{}", rules);

    for _ in 0..steps {
        println!("{tm}");
        tm.step();
    }
}
