use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

const TERMINATE: isize = 99;

pub fn load_code(expand_mult: u32) -> Vec<isize> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens: Vec<isize> = contents
        .trim()
        .split(',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect();

    expand_memory(tokens, expand_mult)
}

fn extract_modes(instruction: isize) -> (u8, u8, u8, u8) {
    let op = instruction % 100;
    let first = (instruction % 1000) / 100;
    let second = (instruction % 10000) / 1000;
    let third = (instruction % 100_000) / 10000;
    (op as u8, first as u8, second as u8, third as u8)
}

pub fn spawn_processes(
    count: isize,
    code: Vec<isize>,
    mut inputs: VecDeque<isize>,
    verbose: bool,
) -> Vec<ProcessState> {
    let mut process_states = vec![];

    for _ in 0..count {
        let mut fresh_state = ProcessState {
            memory: code.clone(),
            input: VecDeque::new(),
            iptr: 0,
            instruction: 0,
            output: VecDeque::new(),
            relative_base: 0,
            verbose,
            instruction_count: 0,
        };
        if !inputs.is_empty() {
            fresh_state.input.push_back(inputs.pop_front().unwrap());
        }
        process_states.push(fresh_state);
    }
    process_states
}

pub struct ProcessState {
    pub memory: Vec<isize>,
    pub input: VecDeque<isize>,
    iptr: usize,
    instruction: isize,
    pub output: VecDeque<isize>,
    relative_base: isize,
    verbose: bool,
    instruction_count: usize,
}

impl ProcessState {
    pub fn is_terminated(&self) -> bool {
        self.instruction == TERMINATE
    }
    pub fn get_instruction_count(&mut self) -> usize {
        let res = self.instruction_count;
        self.instruction_count = 0;
        res
    }
    fn read_write_mem(&mut self, mode: u8, pointer: usize, value: Option<isize>) -> isize {
        //let mode = if value.is_some() { 2 } else { mode };
        let address = match mode {
            0 => self.memory[pointer] as usize,
            1 => pointer as usize,
            2 => (self.relative_base + self.memory[pointer]) as usize,
            _ => panic!("bad mode {} @ {}", mode, pointer),
        };
        match value {
            Some(v) => {
                if self.verbose {
                    println!("{}> storing value {} to {}", mode, v, address);
                }
                self.memory[address] = v;
                v
            }
            None => {
                if self.verbose {
                    println!(
                        "{}> reading value {} from {}",
                        mode, self.memory[address], address
                    );
                }
                self.memory[address]
            }
        }
    }

    pub fn run_to_interrupt(&mut self, min_outputs: usize) {
        while self.memory[self.iptr] != TERMINATE {
            let instruction = self.memory[self.iptr];
            self.instruction_count += 1;
            let (op, mode_1, mode_2, mode_3) = extract_modes(instruction);
            //println!("--> running {} @ {}", instruction, self.iptr);
            match op {
                1 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    let b = self.read_write_mem(mode_2, self.iptr + 2, None);
                    let _ = self.read_write_mem(mode_3, self.iptr + 3, Some(a + b));
                    self.iptr += 4;
                }
                2 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    let b = self.read_write_mem(mode_2, self.iptr + 2, None);
                    let _ = self.read_write_mem(mode_3, self.iptr + 3, Some(a * b));
                    self.iptr += 4;
                }
                3 => {
                    let input_value = self.input.pop_front().expect("need input");
                    let _ = self.read_write_mem(mode_1, self.iptr + 1, Some(input_value));
                    self.iptr += 2;
                }
                4 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    if self.verbose {
                        println!("4 @ {} ========= output> {}", self.iptr, a);
                    }
                    self.output.push_back(a);
                    self.iptr += 2;
                    // interrupt out of here if we have enough output
                    if self.output.len() >= min_outputs {
                        break;
                    }
                }
                5 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    if a != 0 {
                        self.iptr = self.read_write_mem(mode_2, self.iptr + 2, None) as usize;
                        if self.verbose {
                            println!("jumping to {}", self.iptr);
                        }
                    } else {
                        self.iptr += 3;
                    }
                }
                6 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    if a == 0 {
                        self.iptr = self.read_write_mem(mode_2, self.iptr + 2, None) as usize;
                        if self.verbose {
                            println!("jumping to {}", self.iptr);
                        }
                    } else {
                        self.iptr += 3;
                    }
                }
                7 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    let b = self.read_write_mem(mode_2, self.iptr + 2, None);
                    if a < b {
                        let _ = self.read_write_mem(mode_3, self.iptr + 3, Some(1));
                    } else {
                        let _ = self.read_write_mem(mode_3, self.iptr + 3, Some(0));
                    }
                    self.iptr += 4;
                }
                8 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    let b = self.read_write_mem(mode_2, self.iptr + 2, None);
                    if a == b {
                        let _ = self.read_write_mem(mode_3, self.iptr + 3, Some(1));
                    } else {
                        let _ = self.read_write_mem(mode_3, self.iptr + 3, Some(0));
                    }
                    self.iptr += 4;
                }
                9 => {
                    let a = self.read_write_mem(mode_1, self.iptr + 1, None);
                    self.relative_base += a;
                    self.iptr += 2;
                }

                _ => {
                    panic!("illegal instruction {} @ {}", instruction, self.iptr);
                }
            }
        }
        self.instruction = self.memory[self.iptr];
    }
}

fn expand_memory(mut memory: Vec<isize>, times: u32) -> Vec<isize> {
    let memory_size = memory.len();
    for _ in 0..memory_size * times as usize {
        memory.push(0);
    }
    memory
}
