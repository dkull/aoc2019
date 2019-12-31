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

fn convert(pos: isize, cards: isize) -> isize {
    if pos < 0 {
        cards + pos
    } else {
        -(cards - pos)
    }
}

fn both(pos: isize, cards: isize) -> (isize, isize) {
    let c = convert(pos, cards);
    if pos > 0 {
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

fn abs_pos(pos: isize, n_cards: isize) -> isize {
    if pos < 0 {
        n_cards + pos
    } else {
        pos
    }
}

fn run_steps(steps: &Vec<Step>, n_cards: isize) -> (isize, isize) {
    let mut modulus = 1;
    let mut offset = 0;
    let mut direction_right = true;

    for step in steps {
        match step {
            Step::CUT(n) => {
                let n = abs_pos(*n, n_cards);
                println!("cut {} offset was {} now {}", n, offset, offset + n);
                offset += n;
            }
            Step::DEAL(n) => {
                println!("deal {} modulus was {} now {}", n, modulus, modulus * n);
                modulus *= n;
                modulus %= n_cards - 1
            }
            Step::NEWSTACK => {
                println!(
                    "nestack dir was {} now {}",
                    direction_right, !direction_right
                );
                direction_right = !direction_right;
            }
        }
    }
    (modulus, offset)
}

fn main() {
    let target_pos = 2020;
    let steps = load_steps();

    let n_cards: usize = 119_315_717_514_047;
    let iterations: usize = 101_741_582_076_661;
    // too low: 25_310_464_947_432

    let n_cards: usize = 10007;
    let iterations = 1;

    let mut offset = 0;
    for iter in 0..iterations {
        let (modulus, offset) = run_steps(&steps, n_cards as isize);
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
