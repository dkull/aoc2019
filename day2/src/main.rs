use std::fs::File;
use std::io::prelude::*;

const TERMINATE: i64 = 99;

fn process(pointer: usize, memory: Vec<i64>) -> (i64, i64, usize) {
    let (a, b, out) = (
        memory[memory[pointer + 1] as usize],
        memory[memory[pointer + 2] as usize],
        memory[pointer + 3] as usize,
    );
    (a, b, out)
}

fn run_machine(mut code: Vec<i64>, replacements: Vec<(usize, i64)>) -> i64 {
    let memory = &mut code;
    for (offset, value) in replacements {
        memory[offset] = value;
    }
    let mut pointer = 0;
    while memory[pointer] != TERMINATE {
        let instruction = memory[pointer];
        match instruction {
            1 => {
                let (a, b, out) = process(pointer, memory.to_vec());
                memory[out] = a + b;
                pointer += 4;
            }
            2 => {
                let (a, b, out) = process(pointer, memory.to_vec());
                memory[out] = a * b;
                pointer += 4;
            }
            _ => {
                panic!("not a good instruction");
            }
        }
    }
    memory[0]
}

fn expect_output(mut code: Vec<i64>, expect: i64) -> Result<(i64, i64), String> {
    for a in 0..99 {
        for b in 0..99 {
            let result = run_machine(code.clone(), vec![(1, a), (2, b)]);
            if result == expect {
                return Ok((a, b));
            }
        }
    }
    Err("no result found".into())
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|c| {
            println!(">{}", c);
            c.parse::<i64>().unwrap()
        })
        .collect();

    let p1 = run_machine(tokens.clone(), vec![(1, 12), (2, 2)]);
    println!("result p1: {}", p1);

    let p2 = expect_output(tokens, 19_690_720);
    match p2 {
        Ok((a, b)) => {
            println!("result p2: {}", a * 100 + b);
        }
        Err(msg) => {
            println!("result p2 not found");
        }
    }

    Ok(())
}
