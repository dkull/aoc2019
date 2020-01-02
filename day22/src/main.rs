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

fn move_card_to(card_pos: isize, steps: &[Step], n_cards: isize) -> isize {
    fn predict_cut(pos: isize, cut: isize) -> isize {
        pos - cut
    }
    fn predict_deal(pos: isize, deal: isize, n_cards: isize) -> isize {
        pos * deal % n_cards
    }
    fn predict_newstack(pos: isize) -> isize {
        -(pos + 1)
    }

    //println!("moving a card to {}", card_pos);
    let mut card_pos = card_pos;
    for step in steps {
        card_pos = match step {
            Step::CUT(n) => predict_cut(card_pos, *n),
            Step::DEAL(n) => predict_deal(card_pos, *n, n_cards),
            Step::NEWSTACK => predict_newstack(card_pos),
        };
        // has to be positive since i removed all negative handling
        card_pos = both(card_pos, n_cards).0;
        //println!("{:?} caused move to {}", step, card_pos);
    }
    card_pos
}

fn generate_from_numbers(modu: isize, offset: isize, cards: isize, reverse: bool) -> Vec<isize> {
    println!("generating with mod {} offs {}", modu, offset);
    let mut out_vec = vec![];
    for _ in 0..cards {
        out_vec.push(0);
    }
    if !reverse {
        for i in 0..cards {
            out_vec[(((i * modu) + offset) % cards) as usize] = i;
        }
    } else {
        for i in 0..cards {
            let mut index = offset - (i * modu) % cards;
            if index < 0 {
                index += cards;
            }
            index %= cards;
            out_vec[index as usize] = i;
        }
    }
    out_vec
}

fn generate_from_steps(steps: &[Step], cards: isize) -> Vec<isize> {
    let mut offset = 0isize;
    let mut modulus = 0isize;
    let mut reverse = false;
    for step in steps {
        match step {
            Step::CUT(n) => offset = cards - *n,
            Step::DEAL(n) => modulus = *n,
            Step::NEWSTACK => reverse = true,
        }
    }
    generate_from_numbers(modulus, offset, cards, reverse)
}

fn reduce_steps(steps: &[Step], n_cards: isize, iterations: isize) -> (isize, isize, Vec<Step>) {
    extern crate primes;

    let single_steps: Vec<Step> = steps.to_vec();
    let mut minus_one = false;

    let mut steps: Vec<Step> = steps.to_vec();

    let mut modulus = 1;
    let mut offset = 0;

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

    /*println!(
        "iteration {} has factors {:?} minus one? {}",
        iterations, iteration_factors, minus_one
    );*/

    for (i, factor) in iteration_factors.iter().enumerate() {
        // we have to repeat the steps num times in factor
        //println!("doing factor {} on {:?}", factor, steps);

        modulus = 1;
        offset = 0;

        // sanity check
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
                //println!("ran {:?} offset {} modulus {}", step, offset, modulus);
            }
        }

        // we did the number of ops in factor, create new intermediate repr
        //println!("found descriptor modulus {} offset {}", modulus, offset);
        let new_steps = numbers_to_steps(modulus, offset, n_cards);
        /*println!(
            "reduced {:?} down to {:?} in {} factor loops",
            steps, new_steps, factor
        );*/
        /*println!(
            "generated from reduced steps: {:?}",
            generate_from_steps(&new_steps, n_cards )
        );*/
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

fn _mod_pow(base: isize, exp: isize, modulus: isize) -> isize {
    extern crate num_bigint;
    extern crate num;
    extern crate num_traits;

    use num_bigint::BigInt;
    use num_traits::cast::ToPrimitive;

    let b: BigInt = base.into();
    let e: BigInt = exp.into();
    let m: BigInt = modulus.into();
    let res: BigInt = b.modpow(&e, &m);
    res.to_isize().unwrap()
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

fn check(data: &[isize], _offset: isize, modu: isize) -> bool {
    let zero = data.iter().position(|d| d == &0).unwrap() as isize;
    let one = data.iter().position(|d| d == &1).unwrap() as isize;
    /*println!(
        "CHECK zero {} one {} offset {} modulus {}",
        zero, one, _offset, modu
    );*/

    zero == _offset && (_offset + modu) % data.len() as isize == one
}

fn track_movement(steps: &[Step], pos: isize, iterations: isize) {
    // foo
}

fn main() {
    let _target_pos = 2020;
    let n_cards: isize = 119_315_717_514_047;
    let iterations: isize = 101_741_582_076_661;
    // too low:  25_310_464_947_432
    // too hig: 118_781_300_053_829

    let _target_pos = 20;
    let n_cards: isize = 31;
    let iterations: isize = 100;

    let steps = load_steps(n_cards);

    if iterations >= 100_000 {
        let (offset, modulus, steps) = reduce_steps(&steps, n_cards, iterations);
        track_movement(&steps, _target_pos, iterations - n_cards);
        return;
    }

    let mut cards: Vec<isize> = (0..n_cards).collect();
    for iter in 1..=iterations {
        println!("=== iteration {}", iter);
        let mut new_cards = cards.clone();
        for card in 0..n_cards {
            let c = move_card_to(card, &steps, n_cards) % n_cards;
            new_cards[c as usize] = cards[card as usize];
        }

        let res = reduce_steps(&steps, n_cards, iter);
        let correct = check(&new_cards, res.0, res.1);
        let smart_output = generate_from_steps(&res.2, n_cards);
        //let correct = smart_output == new_cards;
        println!("smart {:?}", smart_output);
        println!("brute {:?}", new_cards);
        println!(
            "target at {}",
            new_cards.iter().position(|c| c == &_target_pos).unwrap()
        );
        if !correct {
            println!("ERRROR!");
            //panic!();
        }
        println!("> {}", correct);
        cards = new_cards;
    }
}
