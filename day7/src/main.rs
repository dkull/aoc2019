use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

const TERMINATE: isize = 99;

fn extract_modes(instruction: isize) -> (u8, u8, u8, u8) {
    let op = instruction % 100;
    let first = (instruction % 1000) / 100;
    let second = (instruction % 10000) / 1000;
    let third = (instruction % 100000) / 10000;
    (op as u8, first as u8, second as u8, third as u8)
}

fn get_val(mode: u8, pointer: usize, memory: Vec<isize>) -> isize {
    match mode {
        0 => memory[memory[pointer] as usize],
        1 => memory[pointer] as isize,
        _ => panic!("bad mode {} @ {}", mode, pointer),
    }
}

fn permutate(
    input: &mut Vec<isize>,
    l: usize,
    r: usize,
    output: &mut Vec<VecDeque<isize>>,
) -> Vec<VecDeque<isize>> {
    if l == r {
        output.push(VecDeque::from_iter(input.clone().into_iter()));
    } else {
        for i in l..r + 1 {
            let i = i as usize;

            let _b = input[i];
            input[i] = input[l];
            input[l] = _b;
            permutate(input, l + 1, r, output);

            let _b = input[i];
            input[i] = input[l];
            input[l] = _b;
        }
    }
    return vec![VecDeque::new()];
}

fn spawn_processes(
    count: isize,
    code: Vec<isize>,
    mut inputs: VecDeque<isize>,
) -> Vec<ProcessState> {
    let mut process_states = vec![];

    for i in 0..count {
        let mut fresh_state = ProcessState {
            memory: code.clone(),
            input: VecDeque::new(),
            iptr: 0,
            instruction: 0,
            output: 0,
        };
        fresh_state.input.push_back(inputs.pop_front().unwrap());
        process_states.push(fresh_state);
    }
    process_states
}

struct ProcessState {
    memory: Vec<isize>,
    input: VecDeque<isize>,
    iptr: usize,
    instruction: isize,
    output: isize,
}

impl ProcessState {
    fn run_to_interrupt(&mut self) {
        while self.memory[self.iptr] != TERMINATE {
            let instruction = self.memory[self.iptr];
            let (op, mode_1, mode_2, mode_3) = extract_modes(instruction);
            //println!("running {} @ {}", instruction, self.iptr);
            match op {
                1 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    let b = get_val(mode_2, self.iptr + 2, self.memory.to_vec());
                    let out = get_val(1, self.iptr + 3, self.memory.to_vec());
                    self.memory[out as usize] = a + b;
                    //println!("1@{} {}+{}= {}=>{}", self.iptr, a, b, a + b, out);
                    self.iptr += 4;
                }
                2 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    let b = get_val(mode_2, self.iptr + 2, self.memory.to_vec());
                    let out = get_val(1, self.iptr + 3, self.memory.to_vec());
                    self.memory[out as usize] = a * b;
                    //println!("2@{} {}*{}={}=>@{}", self.iptr, a, b, a * b, out);
                    self.iptr += 4;
                }
                3 => {
                    let out = get_val(1, self.iptr + 1, self.memory.to_vec());
                    self.memory[out as usize] = self.input.pop_front().unwrap();
                    println!(
                        "3 @ {} self.input> {} into {}",
                        self.iptr, self.memory[out as usize], out
                    );
                    self.iptr += 2;
                }
                4 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    println!("4 @ {} output> {}", self.iptr, a);
                    self.output = a;
                    self.iptr += 2;

                    // interrupt out of here
                    break;
                }
                5 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    if a != 0 {
                        let new_iptr =
                            get_val(mode_2, self.iptr + 2, self.memory.to_vec()) as usize;
                        //println!("5@{} -> jumping to {}", self.iptr, new_self.iptr);
                        self.iptr = new_iptr
                    } else {
                        //println!("5@{} -> not jumping", self.iptr);
                        self.iptr += 3;
                    }
                }
                6 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    if a == 0 {
                        let new_iptr =
                            get_val(mode_2, self.iptr + 2, self.memory.to_vec()) as usize;
                        //println!("6@{} -> jumping to {}", self.iptr, new_self.iptr);
                        self.iptr = new_iptr;
                    } else {
                        self.iptr += 3;
                        //println!("6@{} -> not jumping", self.iptr);
                    }
                }
                7 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    let b = get_val(mode_2, self.iptr + 2, self.memory.to_vec());
                    let out = get_val(1, self.iptr + 3, self.memory.to_vec());
                    //println!("7@{} {}<{} -> {}", self.iptr, a, b, out);
                    if a < b {
                        self.memory[out as usize] = 1;
                    } else {
                        self.memory[out as usize] = 0;
                    }
                    self.iptr += 4;
                }
                8 => {
                    let a = get_val(mode_1, self.iptr + 1, self.memory.to_vec());
                    let b = get_val(mode_2, self.iptr + 2, self.memory.to_vec());
                    let out = get_val(1, self.iptr + 3, self.memory.to_vec());
                    //println!("8@{} {}=={} -> {}", self.iptr, a, b, out);
                    if a == b {
                        self.memory[out as usize] = 1;
                    } else {
                        self.memory[out as usize] = 0;
                    }
                    self.iptr += 4;
                }

                _ => {
                    panic!("illegal instruction {} @ {}", instruction, self.iptr);
                }
            }
        }
        self.instruction = self.memory[self.iptr];
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens: Vec<isize> = contents
        .trim()
        .split(',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect();

    // part 1
    /*
    let permutations = {
        let mut output = vec![];
        permutate(&mut vec![0, 1, 2, 3, 4], 0, 4, &mut output);
        output
    };
    let mut best_score = 0;
    for phase_state in permutations.iter() {
        let mut prev_output = 0;
        println!("permutation {:?}", phase_state);
        let mut phase_state = phase_state.clone();
        for _ in 0..5 {
            let mut input = VecDeque::new();
            input.push_back(phase_state.pop_front().unwrap());
            input.push_back(prev_output);
            prev_output = run_machine(tokens.clone(), input.clone());
        }
        if prev_output > best_score {
            best_score = prev_output;
        }
    }
    println!("p1 result: {}", best_score);
    */

    // part 2
    let phase_states = {
        let mut output = vec![];
        permutate(&mut vec![5, 6, 7, 8, 9], 0, 4, &mut output);
        output
    };

    // initialize software states
    let process_count: usize = 5;
    let mut best_score = 0;

    let mut terminated = false;
    for phase_state in phase_states.iter() {
        let mut processes =
            spawn_processes(process_count as isize, tokens.clone(), phase_state.clone());
        let mut phase_state = &phase_state.clone();
        println!("testing phase state {:?}", phase_state);

        let mut prev_output = 0;
        let mut i = 0;
        let mut terminations = 0;
        loop {
            println!("engine {}", i);
            let process_state = &mut processes[i];
            process_state.input.push_back(prev_output);
            process_state.run_to_interrupt();
            if process_state.instruction == TERMINATE {
                terminations += 1;
                println!("TERMINATE {}", terminations);
            }
            prev_output = process_state.output;

            if terminations == process_count {
                break;
            }

            i = (i + 1) % process_count;
        }
        if prev_output > best_score {
            best_score = prev_output;
        }
    }
    println!("p2 result: {}", best_score);

    Ok(())
}
