use std::io::Read;
use std::matches;

struct Brainfuck {
    memory: [u8; 2usize.pow(16)],
    instructions: Vec<char>,
    data_pointer: usize,
    instruction_pointer: usize,
}

impl Brainfuck {
    fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        let file = std::fs::read_to_string(path).expect("Failed to read file");
        let instructions = file
            .chars()
            .filter(|c| matches!(c, '>' | '<' | '+' | '-' | ',' | '.' | '[' | ']'))
            .collect();
        Brainfuck {
            memory: [0; 2usize.pow(16)],
            instructions,
            data_pointer: 0,
            instruction_pointer: 0,
        }
    }
    fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            self.step();
        }
    }
    fn step(&mut self) {
        let command = self.instructions[self.instruction_pointer];
        let value = self.memory[self.data_pointer];
        match command {
            '>' => self.data_pointer += 1,
            '<' => self.data_pointer -= 1,
            '+' => self.memory[self.data_pointer] += 1,
            '-' => self.memory[self.data_pointer] -= 1,
            '.' => print!("{}", value as char),
            ',' => {
                let input = std::io::stdin().bytes().next();
                if let Some(Ok(input)) = input {
                    self.memory[self.data_pointer] = input;
                }
            }
            '[' => {
                if value == 0 {
                    let slice = &self.instructions[self.instruction_pointer..];
                    let matching_bracket = find_matching(('[', ']'), &mut slice.iter());
                    self.instruction_pointer += matching_bracket.unwrap();
                }
            }
            ']' => {
                if value != 0 {
                    let slice = &self.instructions[..self.instruction_pointer];
                    let matching_bracket = find_matching((']', '['), &mut slice.iter().rev());
                    self.instruction_pointer -= matching_bracket.unwrap();
                }
            }
            _ => (),
        }
        self.instruction_pointer += 1;
    }
}

// returns offset of matching bracket's index from current index using slices
fn find_matching<'a>(brackets: (char, char), iter: &mut impl Iterator<Item = &'a char>) -> Option<usize> {
    let mut counter = 1;
    for (i, item) in iter.enumerate() {
        if item == &brackets.0 {
            counter += 1;
        }
        if item == &brackets.1 {
            counter -= 1;
        }
        if counter == 0 {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    let mut brainfuck = Brainfuck::new("./input");
    brainfuck.run();
}
