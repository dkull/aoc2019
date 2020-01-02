use std::collections::HashMap;

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

fn move_card_to(card_pos: isize, steps: &Vec<Step>, n_cards: usize) -> isize {
    fn predict_cut(pos: isize, cut: isize) -> isize {
        pos - cut
    }
    fn predict_deal(pos: isize, deal: isize, n_cards: isize) -> isize {
        pos * deal % n_cards
    }
    fn predict_newstack(pos: isize) -> isize {
        -(pos + 1)
    }

    let orig_pos = card_pos;
    let mut card_pos = card_pos;
    for step in steps {
        card_pos = match step {
            Step::CUT(n) => predict_cut(card_pos, *n),
            Step::DEAL(n) => predict_deal(card_pos, *n, n_cards as isize),
            Step::NEWSTACK => predict_newstack(card_pos),
        };
        // has to be positive since i removed all negative handling
        card_pos = both(card_pos, n_cards as isize).0;
        /*println!(
            "step {:?} caused orig pos {} to move to {}",
            step, orig_pos, card_pos
        );*/
    }
    card_pos
}

fn generate_from_numbers(modu: usize, offset: usize, cards: usize, reverse: bool) -> Vec<usize> {
    println!(
        "generating with mod {} offs {} reversed {}",
        modu, offset, reverse
    );
    let mut out_vec = vec![];
    for i in 0..cards {
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

fn generate_from_steps(steps: &Vec<Step>, cards: usize) -> Vec<usize> {
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
    steps: &Vec<Step>,
    n_cards: isize,
    iterations: isize,
) -> (isize, isize, bool, Vec<Step>) {
    extern crate primes;

    let single_steps = steps.clone();
    let mut minus_one = false;

    let mut steps: Vec<Step> = steps.clone();

    let mut modulus = 1;
    let mut offset = 0;
    let mut reversed = false;

    let mut iteration_factors = primes::factors(iterations as u64);
    if iteration_factors.len() == 1 && iteration_factors[0] > 1_000_000 {
        minus_one = true;
        iteration_factors = primes::factors(iterations as u64 - 1);
    }
    if iterations == 1 {
        iteration_factors = vec![1];
    }

    println!(
        "iteration {} has factors {:?} minus one? {}",
        iterations, iteration_factors, minus_one
    );

    for factor in iteration_factors {
        // we have to repeat the steps num times in factor
        //println!("doing factor {} on {:?}", factor, steps);
        modulus = 1;
        offset = 0;
        reversed = false;

        assert!(factor < 10_000_000);
        for f in 0..factor {
            // do each step
            //println!("-- factor nth {}", f);
            for step in &steps {
                match step {
                    Step::CUT(n) => {
                        let n = abs_pos(*n, n_cards);
                        offset -= n;
                        //println!("cutting ({}) {} to {}", reversed, n, offset);
                    }
                    Step::DEAL(n) => {
                        let n = abs_pos(*n, n_cards);
                        modulus *= n;
                        modulus %= n_cards;
                        if offset > 0 {
                            if !reversed {
                                offset *= n;
                            //println!("mult offset {} -> {}", n, offset);
                            } else {
                                offset = (offset * n) % n_cards;
                                //println!("mult_r offset {} -> {}", n, offset);
                            }
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
        println!(
            "generated from reduced steps: {:?}",
            generate_from_steps(&new_steps, n_cards as usize)
        );
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

fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    extern crate num_bigint;
    extern crate num;
    extern crate num_traits;

    use num_bigint::BigUint;
    use num_traits::cast::ToPrimitive;

    let b: BigUint = base.into();
    let e: BigUint = exp.into();
    let m: BigUint = modulus.into();
    let res: BigUint = b.modpow(&e, &m).into();
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

fn mods_needed(a: usize, b: usize, m: usize) -> usize {
    let mut val = 0;
    for i in 0..m {
        val += a;
        val %= m;
        if val == b {
            return i;
        }
    }
    0
}

fn main() {
    let target_pos = 2020;
    let n_cards: usize = 119_315_717_514_047;
    let iterations: usize = 101_741_582_076_661;
    // too low:  25_310_464_947_432
    // too hig: 118_781_300_053_829

    let target_pos = 1;
    let n_cards: usize = 17;
    let iterations = 10;

    let steps = load_steps(n_cards);

    let mut cards: Vec<usize> = if n_cards < 100_000 {
        (0..n_cards).collect()
    } else {
        (0..1).collect()
    };

    //modulus = mod_pow(modulus as usize, iterations, n_cards);
    //offset = mod_mult(offset as usize, iterations, n_cards);

    if iterations >= 100_000 {
        reduce_steps(&steps, n_cards as isize, iterations as isize);
        return;
    }

    for iter in 1..=iterations {
        println!("=== iteration {}", iter);
        let res = reduce_steps(&steps, n_cards as isize, iter as isize);
        let smart_output = generate_from_steps(&res.3, n_cards);

        let mut new_cards = cards.clone();
        for card in 0..n_cards {
            let c = move_card_to(card as isize, &steps, n_cards as usize) % n_cards as isize;
            new_cards[c as usize] = cards[card as usize];
        }
        let zero_pos = new_cards.iter().position(|c| c == &0usize);

        println!("brute force > {} {:?} {:?}", iter, new_cards, zero_pos);
        //assert_eq!(smart_output, new_cards);
        cards = new_cards;
    }
}

/*
        3-> 0 7 4 1 8 5 2 9 6 3
        7-> 0 1 2 3 4 5 6 7 8 9

        2*7-> 0 2 4 6 8 1 3 5 7

        4-> 0 7 5 3 1 8 6 4 2 -> 2-> 0 0 2 0 4 0 0 0
        7-> 0 1 2 3 4 5 6 7 8

        4-> 0 7 5 3 1 8 6 4 2
        5-> 0 5 1 6 2 7 3 8 4  ->  5-> 0 1 0 0 0 5 0 0 0
        7-> 0 2 4 6 8 1 3 5 7  -> r7-> 0 0 0 3 0 5 0 7 0
        2-> 0 1 2 3 4 5 6 7 8

        7-> 0 2 4 6 8 1 3 5 7
reverse 0 7 5 3 1 8 6 4 2
        4-> 0 4 8 3 7 2 6 1 5 ==^ (5)

        0 4 8 3 7 2 6 1 5
        2 6 1 5 0 4 8 3 7

        0 1 2 3 4 5 6 7 8

*/
