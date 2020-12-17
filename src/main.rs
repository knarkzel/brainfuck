#[derive(Debug)]
struct State {
    memory: [u8; 2usize.pow(16)],
    instructions: Vec<char>,
    data_pointer: usize,
    instruction_pointer: usize,
}

impl State {
    fn new() -> Self {
        State {
            memory: [0; 2usize.pow(16)],
            instructions: vec![],
            data_pointer: 0,
            instruction_pointer: 0,
        }
    }
    fn load<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        let file = std::fs::read_to_string(path).expect("Failed to read file");
        self.instructions = file
            .trim()
            .chars()
            .filter_map(|c| match c {
                '>' | '<' | '+' | '-' | ',' | '.' | '[' | ']' => Some(c),
                _ => None,
            })
            .collect();
        self
    }
    fn should_run(&self) -> bool {
        self.instruction_pointer < self.instructions.len()
    }
    fn parse(&mut self) {
        let command = self.instructions[self.instruction_pointer];
        let value = self.memory[self.data_pointer];
        match command {
            '>' => self.data_pointer += 1,
            '<' => self.data_pointer -= 1,
            '+' => self.memory[self.data_pointer] += 1,
            '-' => self.memory[self.data_pointer] -= 1,
            ',' => {
                self.memory[self.data_pointer] = self.instructions[self.instruction_pointer + 1] as u8;
                self.instruction_pointer += 1;
            }
            '.' => print!("{}", value as char),
            '[' => {
                if value == 0 {
                    let mut counter = 1;
                    let mut slice = &self.instructions[self.instruction_pointer..];
                    loop {
                        let index = slice.iter().position(|c| c == &']').unwrap();
                        counter += self.instructions[self.instruction_pointer..index]
                            .iter()
                            .map(|c| match c {
                                '[' => 1,
                                ']' => -1,
                                _ => 0,
                            })
                            .sum::<isize>();
                        if counter > 0 {
                            slice = &self.instructions[index..];
                        } else {
                            self.instruction_pointer = index;
                            break;
                        }
                    }
                }
            }
            ']' => {
                if value != 0 {
                    let mut counter = 1;
                    let mut slice = &self.instructions[..self.instruction_pointer];
                    loop {
                        let index = slice.iter().rposition(|c| c == &'[').unwrap();
                        counter += self.instructions[index..self.instruction_pointer]
                            .iter()
                            .map(|c| match c {
                                '[' => -1,
                                ']' => 1,
                                _ => 0,
                            })
                            .sum::<isize>();
                        if counter > 0 {
                            slice = &self.instructions[..index];
                        } else {
                            self.instruction_pointer = index;
                            break;
                        }
                    }
                }
            }
            _ => (),
        }
        self.instruction_pointer += 1;
    }
}

fn main() {
    let mut state = State::new().load("./input");
    while state.should_run() {
        state.parse();
    }
}
