use std::fmt::{Display, Formatter, Result};

pub trait Memory {
    fn new(fill: u8) -> Self;
    fn read_memory(&self) -> u8;
    fn write_memory(&mut self, symbol: u8);
    fn move_head(&mut self, movement: u8);
    fn get_movements_by_name(&self) -> &[&str];
}

pub struct Memory1D<const WIDTH: usize> {
    memory: [u8; WIDTH],
    tapehead: [usize; 1],
}

impl<const WIDTH: usize> Memory for Memory1D<WIDTH> {
    fn new(fill: u8) -> Self {
        Memory1D {
            memory: [fill; WIDTH],
            tapehead: [0],
        }
    }
    fn read_memory(&self) -> u8 {
        self.memory[self.tapehead[0]]
    }
    fn write_memory(&mut self, symbol: u8) {
        self.memory[self.tapehead[0]] = symbol;
    }
    fn move_head(&mut self, movement: u8) {
        match movement {
            0 => self.tapehead[0] = (self.tapehead[0] + WIDTH - 1) % WIDTH,
            1 => self.tapehead[0] = (self.tapehead[0] + 1) % WIDTH,

            _ => {}
        }
    }
    fn get_movements_by_name(&self) -> &[&str] {
        &["left", "right", "stay"]
    }
}

impl<const WIDTH: usize> Display for Memory1D<WIDTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for s in 0..WIDTH {
            if s == self.tapehead[0] {
                write!(f, "[{}]", self.memory[s]).unwrap();
            } else {
                write!(f, " {} ", self.memory[s]).unwrap();
            }
        }
        Ok(())
    }
}

pub struct Memory2D<const WIDTH: usize, const HEIGHT: usize> {
    memory: [[u8; WIDTH]; HEIGHT],
    tapehead: [usize; 2],
}

impl<const WIDTH: usize, const HEIGHT: usize> Memory for Memory2D<WIDTH, HEIGHT> {
    fn new(fill: u8) -> Self {
        Memory2D {
            memory: [[fill; WIDTH]; HEIGHT],
            tapehead: [0, 0],
        }
    }
    fn read_memory(&self) -> u8 {
        self.memory[self.tapehead[0]][self.tapehead[1]]
    }
    fn write_memory(&mut self, symbol: u8) {
        self.memory[self.tapehead[0]][self.tapehead[1]] = symbol;
    }
    fn move_head(&mut self, movement: u8) {
        match movement {
            0 => self.tapehead[0] = (self.tapehead[0] + HEIGHT - 1) % HEIGHT,
            1 => self.tapehead[0] = (self.tapehead[0] + 1) % HEIGHT,

            2 => self.tapehead[1] = (self.tapehead[1] + WIDTH - 1) % WIDTH,
            3 => self.tapehead[1] = (self.tapehead[1] + 1) % WIDTH,
            _ => {}
        }
    }
    fn get_movements_by_name(&self) -> &[&str] {
        &["up", "down", "left", "right", "stay"]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Memory2D<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for s_row in 0..HEIGHT {
            for s in 0..WIDTH {
                if s_row == self.tapehead[0] && s == self.tapehead[1] {
                    write!(f, "[{}]", self.memory[s_row][s]).unwrap();
                } else {
                    write!(f, " {} ", self.memory[s_row][s]).unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

pub struct Memory3D<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> {
    memory: [[[u8; WIDTH]; HEIGHT]; DEPTH],
    tapehead: [usize; 3],
}

impl<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> Memory
    for Memory3D<WIDTH, HEIGHT, DEPTH>
{
    fn new(fill: u8) -> Self {
        Memory3D {
            memory: [[[fill; WIDTH]; HEIGHT]; DEPTH],
            tapehead: [0, 0, 0],
        }
    }
    fn read_memory(&self) -> u8 {
        self.memory[self.tapehead[0]][self.tapehead[1]][self.tapehead[2]]
    }
    fn write_memory(&mut self, symbol: u8) {
        self.memory[self.tapehead[0]][self.tapehead[1]][self.tapehead[2]] = symbol;
    }
    fn move_head(&mut self, movement: u8) {
        match movement {
            0 => self.tapehead[0] = (self.tapehead[0] + DEPTH - 1) % DEPTH,
            1 => self.tapehead[0] = (self.tapehead[0] + 1) % DEPTH,

            2 => self.tapehead[1] = (self.tapehead[1] + HEIGHT - 1) % HEIGHT,
            3 => self.tapehead[1] = (self.tapehead[1] + 1) % HEIGHT,

            4 => self.tapehead[2] = (self.tapehead[2] + WIDTH - 1) % WIDTH,
            5 => self.tapehead[2] = (self.tapehead[2] + 1) % WIDTH,

            _ => {}
        }
    }
    fn get_movements_by_name(&self) -> &[&str] {
        &["back", "forward", "up", "down", "left", "right", "stay"]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> Display
    for Memory3D<WIDTH, HEIGHT, DEPTH>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for s_row in 0..HEIGHT {
            for s_slice in 0..DEPTH {
                for s in 0..WIDTH {
                    if s_row == self.tapehead[1]
                        && s_slice == self.tapehead[0]
                        && s == self.tapehead[2]
                    {
                        write!(f, "[{}]", self.memory[s_slice][s_row][s]).unwrap();
                    } else {
                        write!(f, " {} ", self.memory[s_slice][s_row][s]).unwrap();
                    }
                }
                write!(f, "   ").unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}
