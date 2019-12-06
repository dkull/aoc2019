use std::fs::File;
use std::io::prelude::*;

const TERMINATE: i64 = 99;

fn extract_modes(instruction: i64) -> (u8, u8, u8, u8) {
    let op = instruction % 100;
    let first = (instruction % 1000) / 100;
    let second = (instruction % 10000) / 1000;
    let third = (instruction % 100000) / 10000;
    (op as u8, first as u8, second as u8, third as u8)
}

fn get_val(mode: u8, pointer: usize, memory: Vec<i64>) -> i64 {
    match mode {
        0 => memory[memory[pointer] as usize],
        1 => memory[pointer] as i64,
        _ => panic!("bad mode {} @ {}",  mode, pointer)
    }
}

fn run_machine(mut code: Vec<i64>, input: Vec<i64>) {
    let memory = &mut code;
    let mut pointer = 0;

    while memory[pointer] != TERMINATE {
        let instruction = memory[pointer];
        let (op, mode_1, mode_2, mode_3) = extract_modes(instruction);
        //println!("running {} @ {}", instruction, pointer);
        match op {
            1 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                let b = get_val(mode_2, pointer + 2, memory.to_vec());
                let out = get_val(1, pointer + 3, memory.to_vec());
                memory[out as usize] = a + b;
                println!("1@{} {}+{}= {}=>{}", pointer, a, b, a+b, out);
                pointer += 4;
            }
            2 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                let b = get_val(mode_2, pointer + 2, memory.to_vec());
                let out = get_val(1, pointer + 3, memory.to_vec());
                memory[out as usize] = a * b;
                println!("2@{} {}*{}={}=>@{}", pointer, a, b, a*b, out);
                pointer += 4;
            }
            3 => {
                let out = get_val(1, pointer + 1, memory.to_vec());
                memory[out as usize] = *input.iter().nth(0).unwrap();
                println!("3@{} read {} into {}", pointer, memory[out as usize], out);
                pointer += 2;
            }
            4 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                println!("4@{} output> {}", pointer, a);
                pointer += 2;
            }
            5 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                if a != 0 {
                    let new_pointer = get_val(mode_2, pointer + 2, memory.to_vec()) as usize;
                    println!("5@{} -> jumping to {}", pointer, new_pointer);
                    pointer = new_pointer
                } else {
                    println!("5@{} -> not jumping", pointer);
                    pointer += 3;
                }
            }
            6 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                if a == 0 {
                    let new_pointer = get_val(mode_2, pointer + 2, memory.to_vec()) as usize;
                    println!("6@{} -> jumping to {}", pointer, new_pointer);
                    pointer = new_pointer;
                } else {
                    pointer += 3;
                    println!("6@{} -> not jumping", pointer);
                }
            }
            7 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                let b = get_val(mode_2, pointer + 2, memory.to_vec());
                let out = get_val(1, pointer + 3, memory.to_vec());
                println!("7@{} {}<{} -> {}", pointer, a, b, out);
                if a < b {
                    memory[out as usize] = 1;
                } else {
                    memory[out as usize] = 0;
                }
                pointer += 4;
            }
            8 => {
                let a = get_val(mode_1, pointer + 1, memory.to_vec());
                let b = get_val(mode_2, pointer + 2, memory.to_vec());
                let out = get_val(1, pointer + 3, memory.to_vec());
                println!("8@{} {}=={} -> {}", pointer, a, b, out);
                if a == b {
                    memory[out as usize] = 1;
                } else {
                    memory[out as usize] = 0;
                }
                pointer += 4;
            }

            _ => {
                panic!("illegal instruction {} @ {}", instruction, pointer);
            }
        }
    }

    //println!("mem @ {} == {}, exiting", pointer, memory[pointer]);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|c| {
            c.parse::<i64>().unwrap()
        })
        .collect();

    run_machine(tokens.clone(), vec![1]);
    println!();
    run_machine(tokens.clone(), vec![5]);
    Ok(())
}
