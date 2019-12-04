use std::fs::File;
use std::io::prelude::*;

fn check_repeat(data: &str) -> (bool, bool) {
    let mut running_repeat_count = 0;
    let mut seen_p1_repeat = false;
    let mut seen_p2_repeat = false;
    let mut prevchar: Option<char> = None;
    for c in data.chars() {
        if let Some(pc) = prevchar {
            if pc == c {
                running_repeat_count += 1;
                seen_p1_repeat = true;
            } else {
                if running_repeat_count == 1 {
                    seen_p2_repeat = true;
                }
                running_repeat_count = 0;
            }
        }
        prevchar = Some(c);
    }
    if running_repeat_count == 1 {
        seen_p2_repeat = true;
    }
    return (seen_p1_repeat, seen_p2_repeat);
}

fn check_no_decrease(data: &str) -> bool {
    let mut seen_decrease = false;
    let mut prevchar: Option<char> = None;
    for c in data.chars() {
        if let Some(pc) = prevchar {
            if c < pc {
                seen_decrease = true;
                break;
            }
        }
        prevchar = Some(c);
    }
    return !seen_decrease;
}

fn count_passwords(f: &i32, t: &i32) -> (i32, i32) {
    let mut p1_count = 0;
    let mut p2_count = 0;
    for i in *f..*t {
        let repr = format!("{}", i);
        let (repeats_p1, repeats_p2) = check_repeat(&repr);
        let no_decrease = check_no_decrease(&repr);
        if repeats_p1 && no_decrease {
            p1_count += 1;
        }
        if repeats_p2 && no_decrease {
            p2_count += 1;
        }
    }
    (p1_count, p2_count)
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens: Vec<i32> = contents
        .trim()
        .split('-')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let from = tokens.iter().nth(0).unwrap();
    let to = tokens.iter().nth(1).unwrap();

    let result = count_passwords(from, to);
    println!(">result {:?}", result);

    Ok(())
}
