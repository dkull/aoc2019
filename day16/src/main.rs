use std::fs::File;
use std::io::prelude::*;

pub fn load_code() -> Vec<isize> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens: Vec<isize> = contents
        .trim()
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();
    tokens
}

//fn gen_sequence(nth: usize) -> std::iter::Cycle<std::slice::Iter<isize>> {}

fn main() {
    let sequence = vec![0, 1, 0, -1];
    let mut code = load_code();

    code = code
        .iter()
        .cycle()
        .take(1 * code.len())
        .map(|c| c.clone())
        .collect::<Vec<isize>>();

    println!("code length is {}", code.len());

    //println!("{:?}", code);
    println!("Hello, world!");
    for phase in 0..100 {
        code = (0..code.len())
            .map(|nth| {
                let mut mult_cycle = vec![];
                sequence
                    .iter()
                    .for_each(|s| (1..=nth + 1).for_each(|_| mult_cycle.push(s)));
                let seq_cycle = mult_cycle.iter().cycle().skip(1).take(code.len());
                let calced = code
                    .iter()
                    .zip(seq_cycle)
                    .map(|(a, b)| a * *b)
                    .sum::<isize>();
                let res = calced.abs() % 10;
                println!("phase {} nth {}/{} = {}", phase, nth, code.len(), res);
                res
            })
            .collect();
        println!("phase {} done: {}", phase, code.len());

        if phase == 99 {
            println!(
                "> {} {:?}",
                phase,
                code.iter().take(8).collect::<Vec<&isize>>()
            );
        }
    }
}
