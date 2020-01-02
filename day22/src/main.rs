#[derive(Debug, Clone)]
enum Step {
    DEAL(isize),
    CUT(isize),
    NEWSTACK,
}

fn load_steps(cards: isize) -> Vec<Step> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .trim()
        .split('\n')
        .filter(|l| !l.contains('#'))
        .map(|c| {
            if c.contains("deal with increment") {
                let val = c.split(' ').last().unwrap().parse::<isize>().unwrap();
                Step::DEAL(val)
            } else if c.contains("cut") {
                let val = c.split(' ').last().unwrap().parse::<isize>().unwrap();
                if val < 0 {
                    Step::CUT(cards + val)
                } else {
                    Step::CUT(val)
                }
            } else if c.contains("deal into new stack") {
                Step::NEWSTACK
            } else {
                panic!("unkwnown step: {}", c);
            }
        })
        .collect()
}

fn abs_pos(pos: isize, n_cards: isize) -> isize {
    if pos < 0 {
        n_cards + pos
    } else {
        pos
    }
}

fn reduce_steps(steps: &[Step], n_cards: isize, iterations: isize) -> (isize, isize, Vec<Step>) {
    extern crate primes;

    let single_steps: Vec<Step> = steps.to_vec();
    let mut minuses = 0;

    let mut steps: Vec<Step> = steps.to_vec();

    let mut modulus = 1;
    let mut offset = 0;

    let mut iteration_factors = primes::factors(iterations as u64);
    if iteration_factors.is_empty() {
        //println!("forcing factor {}", iterations);
        iteration_factors = vec![iterations as u64];
    } else {
        while iteration_factors.iter().last().unwrap() > &1_000_000 {
            minuses += 1;
            iteration_factors = primes::factors(iterations as u64 - minuses);
        }
    }

    println!(
        "iteration {} has factors {:?} minus ones? {}",
        iterations, iteration_factors, minuses
    );

    for (i, factor) in iteration_factors.iter().enumerate() {
        // we have to repeat the steps num times in factor
        modulus = 1;
        offset = 0;

        // sanity check
        assert!(factor < &10_000_000);
        for f in 0..*factor {
            // do each step
            if minuses > 0 && f == factor - 1 && i == iteration_factors.len() - 1 {
                for _ in 0..minuses {
                    for ss in &single_steps {
                        steps.push(ss.clone());
                    }
                }
            }
            for step in &steps {
                match step {
                    Step::CUT(n) => {
                        offset -= abs_pos(*n, n_cards);
                    }
                    Step::DEAL(n) => {
                        let n = abs_pos(*n, n_cards);
                        modulus = mod_mult(modulus, n, n_cards);
                        offset = mod_mult(offset, n, n_cards);
                    }
                    Step::NEWSTACK => {
                        offset = n_cards - 1 - offset;
                        modulus = n_cards - modulus;
                    }
                }
                if offset < 0 {
                    offset += n_cards;
                }
                offset %= n_cards;
            }
        }

        // we did the number of ops in factor, create new intermediate repr
        let new_steps = numbers_to_steps(modulus, offset, n_cards);
        steps = new_steps.clone();
    }

    (offset, modulus, steps.to_vec())
}

fn numbers_to_steps(modulus: isize, offset: isize, cards: isize) -> Vec<Step> {
    let mut out = vec![];
    out.push(Step::DEAL(modulus));
    if offset > 0 {
        out.push(Step::CUT(cards - offset));
    }
    out
}

fn mod_mult(a: isize, b: isize, m: isize) -> isize {
    extern crate num_bigint;
    extern crate num;
    extern crate num_traits;

    use num_bigint::BigInt;
    use num_traits::cast::ToPrimitive;

    let a: BigInt = a.into();
    let b: BigInt = b.into();
    let res: BigInt = (a * b) % m;
    res.to_isize().unwrap()
}

fn track_movement(steps: &[Step], pos: isize, cards: isize) -> isize {
    let mut pos = pos;
    for step in steps {
        match step {
            Step::CUT(n) => {
                pos += cards - n;
                pos %= cards;
            }
            Step::DEAL(n) => {
                pos *= n;
                pos %= cards;
            }
            _ => panic!("we don't generate those"),
        }
    }
    pos
}

fn main() {
    let _target_pos = 2020;
    let n_cards: isize = 119_315_717_514_047;
    let iterations: isize = 101_741_582_076_661;

    let steps = load_steps(n_cards);

    let (_, _, steps) = reduce_steps(&steps, n_cards, n_cards - iterations - 1);
    let result = track_movement(&steps, _target_pos, n_cards);
    println!("p2: {}", result);
}
