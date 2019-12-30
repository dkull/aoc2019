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
                panic!("unkwnown step");
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

fn round(card_pos: isize, steps: &Vec<Step>, n_cards: isize) -> isize {
    let mut card_pos = card_pos;
    for step in steps {
        card_pos = match step {
            Step::CUT(n) => predict_cut(card_pos, *n),
            Step::DEAL(n) => predict_deal(card_pos, *n, n_cards),
            Step::NEWSTACK => predict_newstack(card_pos),
        };
        // has to be positive since i removed all negative handling
        card_pos = both(card_pos, n_cards as isize).0;
    }
    card_pos
}

fn main() {
    let find_card = 2020;
    let steps = load_steps();

    let n_cards: usize = 119_315_717_514_047;
    let iterations: usize = 101_741_582_076_661;
    // too low: 25310464947432

    //let n_cards: usize = 6101;
    //let iterations = 6091;

    let mut card_pos: isize = find_card;

    let mut wrap_candidates = vec![];
    for i in 1..=10000 {
        let wrap_candidate = n_cards / i;
        if wrap_candidate > iterations {
            continue;
        }
        let wrap_point = iterations / wrap_candidate;
        let wrap_iterations = iterations % (wrap_candidate * wrap_point);
        wrap_candidates.push((wrap_iterations, wrap_candidate));
        println!(
            "predicting wrap point at {} needing {}-({}*{})={} iterations",
            wrap_candidate, iterations, wrap_candidate, wrap_point, wrap_iterations,
        );
    }
    wrap_candidates.sort_by(|a, b| a.0.cmp(&b.0));
    println!("found {} candidates", wrap_candidates.len());

    let mut candidate_index = 0;
    let mut current_candidate = wrap_candidates[candidate_index];
    for iter in 1..=iterations {
        card_pos = round(card_pos, &steps, n_cards as isize);

        if card_pos == find_card {
            println!("{}", card_pos);
            panic!();
        }

        if wrap_candidates[candidate_index].0 == iter {
            while current_candidate.0 == iter {
                candidate_index += 1;
                current_candidate = wrap_candidates[candidate_index];
            }

            let v = wrap_candidates[candidate_index].1;
            println!(" === ");
            println!(
                "testing iter {} with wrap point {} expect {} in {} iterations",
                &iter,
                v,
                find_card,
                v - iter
            );
            /*let mut test_card_pos = card_pos;
            for _ in 0..v - iter {
                test_card_pos = round(test_card_pos, &steps, n_cards as isize);
            }
            println!(
                "? iter {} at {:?} testing {} == {}",
                iter, pair, card_pos, test_card_pos
            );
            if find_card == test_card_pos {
                println!("FOUND WRAPAROUND so answer is {}", card_pos);
                panic!();
            }*/
        }
    }

    println!(
        "answer to cards {} iterations {} => {:?}",
        n_cards,
        iterations,
        both(card_pos, n_cards as isize)
    );
}
