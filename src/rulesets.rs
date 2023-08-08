use rand::Rng;
use std::fmt::{Display, Formatter, Result};

pub struct RuleSets {
    states: u8,
    symbols: u8,
    movements: u8,
    pub rules: Vec<[u8; 5]>,
}

impl RuleSets {
    pub fn new(states: u8, symbols: u8, movements: u8) -> Self {
        RuleSets {
            states,
            symbols,
            movements,
            rules: Vec::new(),
        }
    }
    pub fn possible_rules(&self) -> u128 {
        (self.states as u128).pow(2) * (self.symbols as u128).pow(2) * (self.movements as u128)
    }
    pub fn possible_inputs(&self) -> u128 {
        (self.states as u128) * (self.symbols as u128)
    }
    pub fn possible_rule_sets(&self) -> u128 {
        Self::binom(self.possible_rules(), self.possible_inputs())
    }
    fn binom(n: u128, k: u128) -> u128 {
        let mut res = 1;
        for i in 0..k {
            res = (res * (n - i)) / (i + 1);
        }
        res
    }

    pub fn random_ruleset<R: Rng>(&mut self, rng: &mut R) {
        let mut new_state: u8 = rng.gen_range(0..self.states);
        let mut new_symbol: u8 = rng.gen_range(0..self.symbols);
        let mut move_direction: u8 = rng.gen_range(0..self.movements);

        for current_state in 0..self.states {
            for current_symbol in 0..self.symbols {
                self.rules.push([
                    current_state,
                    current_symbol,
                    (new_state + rng.gen_range(0..self.states)) % self.states,
                    (new_symbol + rng.gen_range(0..self.symbols)) % self.symbols,
                    (move_direction + rng.gen_range(0..self.movements)) % self.movements,
                ]);
                new_state = (new_state + rng.gen_range(0..self.states)) % self.states;
                new_symbol = (new_symbol + rng.gen_range(0..self.symbols)) % self.symbols;
                move_direction =
                    (move_direction + rng.gen_range(0..self.movements)) % self.movements;
            }
        }
    }

    pub fn mutate_ruleset<R: Rng>(&mut self, changes: u8, rng: &mut R) {
        for _ in 0..changes {
            let rule_index = rng.gen_range(0..self.rules.len());
            let change_index = rng.gen_range(0..3);
            match change_index {
                0 => self.rules[rule_index][0] = (self.rules[rule_index][0] + 1) % self.states,
                1 => self.rules[rule_index][1] = (self.rules[rule_index][1] + 1) % self.symbols,
                2 => self.rules[rule_index][2] = (self.rules[rule_index][2] + 1) % self.movements,
                _ => {}
            }
            print!(
                "mutate rule #{} ( {} {} ) -> ( ",
                rule_index, self.rules[rule_index][0], self.rules[rule_index][1]
            );
            let a: String = (0..3)
                .into_iter()
                .filter_map(|el| {
                    if el == change_index {
                        Some(format!("[{}] ", self.rules[rule_index][el]))
                    } else {
                        Some(format!("{} ", self.rules[rule_index][el]))
                    }
                })
                .collect();
            println!("{})", a);
        }
    }
}

impl Display for RuleSets {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{} states, {} symbols", self.states, self.symbols).unwrap();
        write!(
            f,
            "rules of ({} states, {} symbols)-turing machine.\n",
            self.states, self.symbols
        )
        .unwrap();
        for (i, r) in self.rules.iter().enumerate() {
            write!(
                f,
                "   # {:2}: ( {} {} ) -> ( {} {} movement #{} )\n",
                i, r[0], r[1], r[2], r[3], r[4]
            )
            .unwrap();
        }
        Ok(())
    }
}
