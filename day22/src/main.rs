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

fn analyze_steps(steps: &Vec<Step>, pos: isize, n_cards: isize) -> isize {
    let mut val_pos = pos;
    for step in steps.iter().rev() {
        match step {
            Step::NEWSTACK => {
                let pulled_value_pos = n_cards - 1 - val_pos;
                println!("newstack {} => {}", val_pos, pulled_value_pos);
                val_pos = pulled_value_pos;
            }
            Step::CUT(n) => {
                let n = abs_pos(*n, n_cards);

                if n > val_pos {
                    let mut new_val_pos = val_pos - n_cards - n;
                    //let mut new_val_pos = val_pos + n;
                    if new_val_pos < 0 {
                        println!("fix {}", new_val_pos);
                        //new_val_pos = n_cards + (n_cards + new_val_pos - 1)
                        new_val_pos = (n + val_pos) % n_cards;
                        println!("fix to {}", new_val_pos);
                    }
                    println!("cut>({}) {} came from {}", n, val_pos, new_val_pos);
                    val_pos = new_val_pos;
                } else {
                    let mut new_val_pos = (val_pos - 1 - n) % n_cards;
                    new_val_pos = val_pos - (n_cards - n);
                    if new_val_pos < 0 {
                        new_val_pos = n_cards + new_val_pos
                    }
                    println!("cut<({}) {} came from {}", n, val_pos, new_val_pos);
                    val_pos = new_val_pos;
                }
            }
            Step::DEAL(n) => {
                for i in 0..n_cards {
                    let res = i * n % n_cards;
                    if res == val_pos {
                        println!("deal({}) {} came from {}", n, val_pos, i);
                        val_pos = i;
                        break;
                    }
                }
            }
        }
    }
    val_pos
}

fn main() {
    let target_pos = 8;
    let steps = load_steps();

    let n_cards: usize = 119_315_717_514_047;
    let iterations: usize = 101_741_582_076_661;
    // too low: 25_310_464_947_432

    let n_cards: usize = 10;
    let iterations = 1;

    let mut cards: Vec<usize> = (0..n_cards).collect();

    for z in 0..10 {
        let res = analyze_steps(&steps, z as isize, n_cards as isize);
        println!("heur {} {:?}", z, res);
        println!();
    }

    //println!("cards {:?}", cards);
    for iter in 0..iterations {
        let mut new_cards = cards.clone();
        for (i, c) in cards.iter().enumerate() {
            let new_pos = round(i as isize, &steps, n_cards) as usize % n_cards;
            new_cards[new_pos] = *c;
        }
        cards = new_cards;
        println!("{} cards {:?}", iter, cards);
        println!("really at target {}", cards[target_pos as usize]);
    }
}
