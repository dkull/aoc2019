#[derive(Debug, Clone)]
enum Step {
    DEAL(isize),
    CUT(isize),
    NEWSTACK,
}

fn load_steps(cards: usize) -> Vec<Step> {
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
                    Step::CUT(cards as isize + val)
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

fn both(pos: isize, cards: isize) -> (isize, isize) {
    fn convert(pos: isize, cards: isize) -> isize {
        if pos < 0 {
            cards + pos
        } else {
            -(cards - pos)
        }
    }

    let c = convert(pos, cards);
    if pos >= 0 {
        (pos, c)
    } else {
        (c, pos)
    }
}

fn move_card_to(card_pos: isize, steps: &[Step], n_cards: usize) -> isize {
    fn predict_cut(pos: isize, cut: isize) -> isize {
        pos - cut
    }
    fn predict_deal(pos: isize, deal: isize, n_cards: isize) -> isize {
        pos * deal % n_cards
    }
    fn predict_newstack(pos: isize) -> isize {
        -(pos + 1)
    }

    let mut card_pos = card_pos;
    for step in steps {
        card_pos = match step {
            Step::CUT(n) => predict_cut(card_pos, *n),
            Step::DEAL(n) => predict_deal(card_pos, *n, n_cards as isize),
            Step::NEWSTACK => predict_newstack(card_pos),
        };
        // has to be positive since i removed all negative handling
        card_pos = both(card_pos, n_cards as isize).0;
    }
    card_pos
}

fn generate_from_numbers(modu: usize, offset: usize, cards: usize, reverse: bool) -> Vec<usize> {
    println!(
        "generating with mod {} offs {} reversed {}",
        modu, offset, reverse
    );
    let mut out_vec = vec![];
    for _ in 0..cards {
        out_vec.push(0);
    }
    if !reverse {
        for i in 0..cards {
            out_vec[((i * modu) + offset) % cards] = i;
        }
    } else {
        for i in 0..cards {
            let mut index = offset as isize - (i as isize * modu as isize) % cards as isize;
            if index < 0 {
                index = cards as isize + index as isize;
            }
            index %= cards as isize;
            out_vec[index as usize] = i;
        }
    }
    out_vec
}

fn generate_from_steps(steps: &[Step], cards: usize) -> Vec<usize> {
    let mut offset = 0usize;
    let mut modulus = 0usize;
    let mut reverse = false;
    for step in steps {
        match step {
            Step::CUT(n) => offset = cards - *n as usize,
            Step::DEAL(n) => modulus = *n as usize,
            Step::NEWSTACK => reverse = true,
        }
    }
    generate_from_numbers(modulus, offset, cards, reverse)
}

fn reduce_steps(
    steps: &[Step],
    n_cards: isize,
    iterations: isize,
) -> (isize, isize, bool, Vec<Step>) {
    extern crate primes;

    let single_steps: Vec<Step> = steps.to_vec();
    let mut minus_one = false;

    let mut steps: Vec<Step> = steps.to_vec();

    let mut modulus = 1;
    let mut offset = 0;
    let mut reversed = false;

    let mut iteration_factors = primes::factors(iterations as u64);
    if iteration_factors.len() == 1 {
        minus_one = true;
        iteration_factors = primes::factors(iterations as u64 - 1);
    }
    if iteration_factors.is_empty() {
        //println!("forcing factor {}", iterations);
        minus_one = false;
        iteration_factors = vec![iterations as u64];
    }

    println!(
        "iteration {} has factors {:?} minus one? {}",
        iterations, iteration_factors, minus_one
    );

    for (i, factor) in iteration_factors.iter().enumerate() {
        // we have to repeat the steps num times in factor
        //println!("doing factor {} on {:?}", factor, steps);
        modulus = 1;
        offset = 0;
        reversed = false;

        assert!(factor < &10_000_000);
        for f in 0..*factor {
            // do each step
            //println!("-- factor nth {}", f);
            if minus_one && f == factor - 1 && i == iteration_factors.len() - 1 {
                //println!("adding single steps");
                for ss in &single_steps {
                    steps.push(ss.clone());
                }
            }
            for step in &steps {
                match step {
                    Step::CUT(n) => {
                        let n = abs_pos(*n, n_cards);
                        offset -= n;
                        //println!("cutting ({}) {} to {}", reversed, n, offset);
                    }
                    Step::DEAL(n) => {
                        let n = abs_pos(*n, n_cards);
                        modulus = mod_mult(modulus as usize, n as usize, n_cards as usize) as isize;
                        if offset > 0 {
                            offset =
                                mod_mult(offset as usize, n as usize, n_cards as usize) as isize;
                        }
                    }
                    Step::NEWSTACK => {
                        offset = n_cards - 1 - offset;
                        //println!("newstack toggle offset -> {}", offset);
                        reversed = !reversed;
                    }
                }
                if offset < 0 {
                    offset += n_cards;
                    //println!("offset fixed to {}", offset);
                }
                offset %= n_cards;
            }
        }

        // we did the number of ops in factor, create new intermediate repr
        println!(
            "found descriptor [rev {}] modulus {} offset {}",
            reversed, modulus, offset
        );
        let new_steps = numbers_to_steps(modulus, offset, reversed, n_cards);
        println!(
            "reduced {:?} down to {:?} in {} factor loops",
            steps, new_steps, factor
        );
        /*println!(
            "generated from reduced steps: {:?}",
            generate_from_steps(&new_steps, n_cards as usize)
        );*/
        steps = new_steps.clone();
    }

    (offset, modulus, reversed, steps.to_vec())
}

fn numbers_to_steps(modulus: isize, offset: isize, reversed: bool, cards: isize) -> Vec<Step> {
    let mut out = vec![];
    out.push(Step::DEAL(modulus));
    out.push(Step::CUT(cards - offset));
    if reversed {
        out.push(Step::NEWSTACK);
    }
    out
}

fn _mod_pow(base: usize, exp: usize, modulus: usize) -> usize {
    extern crate num_bigint;
    extern crate num;
    extern crate num_traits;

    use num_bigint::BigUint;
    use num_traits::cast::ToPrimitive;

    let b: BigUint = base.into();
    let e: BigUint = exp.into();
    let m: BigUint = modulus.into();
    let res: BigUint = b.modpow(&e, &m);
    res.to_usize().unwrap()
}

fn mod_mult(a: usize, b: usize, m: usize) -> usize {
    extern crate num_bigint;
    extern crate num;
    extern crate num_traits;

    use num_bigint::BigUint;
    use num_traits::cast::ToPrimitive;

    let a: BigUint = a.into();
    let b: BigUint = b.into();
    let res: BigUint = (a * b) % m;
    res.to_usize().unwrap()
}

fn check(data: &[usize], _offset: isize, modu: isize, reversed: bool) -> bool {
    let zero = data.iter().position(|d| d == &0).unwrap();
    let one = data.iter().position(|d| d == &1).unwrap();

    if !reversed {
        (zero + modu as usize) % data.len() == one && _offset == zero as isize
    } else {
        (one + modu as usize) % data.len() == zero
        //&& (_offset == data.len() as isize - 1 - zero as isize)
    }
}

fn main() {
    let _target_pos = 2020;
    let n_cards: usize = 119_315_717_514_047;
    let iterations: usize = 101_741_582_076_661;
    // too low:  25_310_464_947_432
    // too hig: 118_781_300_053_829

    // let n_cards: usize = 10007;
    //let iterations: usize = 13000;

    let steps = load_steps(n_cards);

    let mut cards: Vec<usize> = if n_cards < 100_000 {
        (0..n_cards).collect()
    } else {
        (0..1).collect()
    };

    if iterations >= 100000 {
        reduce_steps(&steps, n_cards as isize, iterations as isize);
        return;
    }

    for iter in 1..=iterations {
        println!("=== iteration {}", iter);
        let mut new_cards = cards.clone();
        for card in 0..n_cards {
            let c = move_card_to(card as isize, &steps, n_cards as usize) % n_cards as isize;
            new_cards[c as usize] = cards[card as usize];
        }

        let res = reduce_steps(&steps, n_cards as isize, iter as isize);
        let correct = check(&new_cards, res.0, res.1, res.2);
        if !correct {
            let smart_output = generate_from_steps(&res.3, n_cards);
            println!("smart {:?}", smart_output);
            println!("brute {:?}", new_cards);
            panic!();
        }
        println!("> {}", correct);
        cards = new_cards;
    }
}
