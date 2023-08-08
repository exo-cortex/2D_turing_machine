use crate::turing_machine_memory::Memory;
use std::fmt::{Display, Formatter, Result};

pub struct TM<M: Memory> {
    state: u8,
    memory: M,
    rules: Vec<[u8; 5]>,
}

impl<M: Memory> TM<M> {
    pub fn new() -> Self {
        TM {
            state: 0,
            memory: M::new(0),
            rules: Vec::new(),
        }
    }
    pub fn insert_rules(&mut self, rules: &Vec<[u8; 5]>) {
        self.rules = rules.clone();
    }

    pub fn step(&mut self) {
        let field = self.memory.read_memory();
        // println!("state: {}, symbol: {}", self.state, field);
        let rule = self
            .rules
            .iter()
            .find(|rule| rule[0] == self.state && rule[1] == field)
            .unwrap();

        self.state = rule[2];
        self.memory.write_memory(rule[3]);
        self.memory.move_head(rule[4]);
    }
}

impl<M: Memory + std::fmt::Display> Display for TM<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "state: {}\n{}", self.state, self.memory)
    }
}
