use std::collections::HashMap;

#[derive(Debug)]
enum Step {
    DEAL(isize),
    CUT(isize),
    NEWSTACK,
}

fn load_steps() -> Vec<Step> {
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
                Step::CUT(val)
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

fn convert(pos: isize, cards: isize) -> isize {
    if pos < 0 {
        cards + pos
    } else {
        -(cards - pos)
    }
}

fn both(pos: isize, cards: isize) -> (isize, isize) {
    let c = convert(pos, cards);
    if pos >= 0 {
        (pos, c)
    } else {
        (c, pos)
    }
}

fn predict_cut(pos: isize, cut: isize) -> isize {
    pos - cut
}
fn predict_deal(pos: isize, deal: isize, n_cards: isize) -> isize {
    pos * deal % n_cards
}
fn predict_newstack(pos: isize) -> isize {
    -(pos + 1)
}

fn round(card_pos: isize, steps: &Vec<Step>, n_cards: usize) -> isize {
    let mut card_pos = card_pos;
    for step in steps {
        let old = card_pos;
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

fn run_steps(
    steps: &Vec<Step>,
    n_cards: isize,
    modulus: isize,
    offset: isize,
) -> (isize, isize, isize, isize) {
    let mut reversed = false;

    let mut modulus = modulus;
    let mut offset = offset;

    let mut added_cuts = 0;
    let mut multiplied_deals = 1;

    for step in steps {
        match step {
            Step::CUT(n) => {
                let old = offset;
                let n = abs_pos(*n, n_cards);
                offset += n;
                offset %= n_cards;
                added_cuts += n;
                //println!("cut {} offset was {} now {}", n, old, offset);
            }
            Step::DEAL(n) => {
                let old = modulus;
                let n = abs_pos(*n, n_cards);
                modulus *= n;
                modulus %= n_cards;

                multiplied_deals *= n;
                multiplied_deals %= n_cards;
                //println!("deal {} modulus was {} now {}", n, old, modulus);
            }
            Step::NEWSTACK => {
                //println!("newstack reversed was {} now {}", reversed, !reversed);
                reversed = !reversed;
            }
        }
    }
    println!("end at mod {} offs {} rev {}", modulus, offset, reversed);
    //(modulus, offset)
    (added_cuts, multiplied_deals, modulus, offset)
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

fn main() {
    let steps = load_steps();

    let target_pos = 2020;
    let n_cards: usize = 119_315_717_514_047;
    let iterations: usize = 101_741_582_076_661;
    // too low: 25_310_464_947_432

    let target_pos = 2;
    let n_cards: usize = 29;
    let iterations = 1;

    let mut cards: Vec<usize> = if n_cards < 1000000 {
        (0..n_cards).collect()
    } else {
        (0..1).collect()
    };

    let mut modulus: usize = 1;
    let mut offset: usize = 0;
    let mut first_modulus: usize = 0;
    let mut first_offset: usize = 0;

    let res = run_steps(&steps, n_cards as isize, modulus as isize, offset as isize);
    let cuts = res.0;
    let deals = res.1;
    println!("cuts {} deals {}", cuts, deals);

    println!("===");

    let pred_modulus = mod_pow(deals as usize, iterations, n_cards);
    let pred_offset = mod_mult(cuts as usize, iterations, n_cards);

    println!(
        "predicting next for iter {}: mod {} offs {}",
        iterations, pred_modulus, pred_offset
    );

    let target_offset = (pred_offset + target_pos) % n_cards;
    println!("{} is at offset {}", target_pos, target_offset);
    let can_fit = n_cards / pred_modulus;
    println!("can fit {:.5} values in one go", can_fit);
    //println!("offset % mod {}", pred_offset % pred_modulus);
    let delta = n_cards % pred_modulus;
    println!("with mod of {}", delta);
    println!("so a number {} places before me is next round", delta);
    println!("the number is larger by {}", can_fit);
    println!("every nr before is me-{} iteration", delta - 1);
    println!("thus is +{}", (delta - 1) * can_fit);

    println!(
        "target is +{} times {} first round first jump [{}]",
        target_offset % pred_modulus,
        target_offset / pred_modulus,
        target_offset > pred_modulus
    );
    for i in 0..7 {
        let res = (pred_modulus * i) % n_cards;
        println!("  mark is {} at {} round", res, i,);
    }

    if iterations < 1000000 {
        /*let res = run_steps(&steps, n_cards as isize, modulus as isize, offset as isize);
        modulus = res.2 as usize;
        offset = res.3 as usize;

        if iter == 0 {
            first_modulus = modulus;
            first_offset = offset;
        }*/
        let mut new_cards = cards.clone();
        for card in 0..n_cards {
            let c = round(card as isize, &steps, n_cards as usize) % n_cards as isize;
            //println!("card {} -> {} {}", card, cards[card as usize], c);
            new_cards[c as usize] = cards[card as usize];
        }
        for card in 0..n_cards {
            println!("> real nth {} == {}", card, new_cards[card],);
        }
        cards = new_cards;
        for iter in 1..=iterations {
            /*for i in 0..target_pos {
                println!("
            }*/
            //let mut delta = (pred_offset + ((can_fit) * pred_modulus)) % n_cards;
            //println!("delta = {}", delta);

            
        }
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
