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

fn main() {
    let cards = 10007;
    let steps = load_steps();
    let mut cards: Vec<usize> = (0..cards).collect();

    for step in steps {
        println!("{:?}", step);
        match step {
            Step::CUT(n) => {
                let n = if n < 0 {
                    cards.len() - (n.abs() as usize)
                } else {
                    n as usize
                };
                let (left, right) = cards.split_at(n);
                let mut right = right.to_vec();
                right.extend(left);
                cards = right.to_vec();
            }
            Step::DEAL(n) => {
                let mut new = cards.clone();
                let mut i = 0;
                for card in &cards {
                    new[i] = *card;
                    i += n as usize;
                    i %= cards.len();
                }
                cards = new;
            }
            Step::NEWSTACK => cards.reverse(),
        }
    }

    if cards.len() <= 100 {
        println!("cards: {:?}", cards);
    }

    println!(
        "p1: {:?}",
        cards.iter().position(|c| *c == 2019usize).unwrap()
    );
}
