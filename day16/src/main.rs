use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;

pub fn load_code() -> Vec<isize> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens: Vec<isize> = contents
        .trim()
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();
    tokens
}

fn calc_seq_val(idx: usize, nth: usize, seq_len: usize, seq: &[isize]) -> isize {
    let p_idx = idx + 1;
    let nth = nth + 1;
    let seq_idx = (p_idx - (p_idx % nth)) / nth % seq_len;
    seq[seq_idx]
}

fn find_block_end(idx: usize, nth: usize) -> isize {
    let nth = nth + 1;
    let block_begin: isize = ((1 + idx as isize) / nth as isize) * nth as isize - 1;
    let block_end = block_begin + nth as isize - 1 as isize;
    block_end as isize
}

fn brute_calc(
    code_len: usize,
    nth: usize,
    seq_len: usize,
    code: &[isize],
    sequence: &[isize],
) -> isize {
    let mut prod_sum = 0isize;
    let mut idx = nth;
    while idx < code_len {
        let seq_val = calc_seq_val(idx, nth, seq_len, sequence);
        prod_sum += code[idx] * seq_val;
        idx += 1;
    }
    prod_sum
}

fn smart_calc(
    code_len: usize,
    nth: usize,
    seq_len: usize,
    code: &[isize],
    sequence: &[isize],
) -> isize {
    let mut delta = 0isize;
    let mut idx = 0;

    let mut prev = 0;
    let mut run_p = 0;
    let mut curr = 0;
    let mut run_c = 0;

    while idx < code_len {
        prev = if run_p > idx {
            prev
        } else {
            calc_seq_val(idx, nth - 1, seq_len, &sequence)
        };
        curr = if run_c > idx {
            curr
        } else {
            calc_seq_val(idx, nth, seq_len, &sequence)
        };

        if prev != curr {
            delta -= code[idx] * prev;
            delta += code[idx] * curr;
        } else {
            run_p = find_block_end(idx, nth - 1) as usize;
            run_c = find_block_end(idx, nth) as usize;
            idx = min(run_p, run_c) as usize;
        }
        idx += 1;
    }
    delta
}

fn main() {
    let sequence = vec![0, 1, 0, -1];
    let seq_len = sequence.len();

    let code = load_code();
    let code_len = code.len();
    let mut code = code
        .into_iter()
        .cycle()
        .take(10000 * code_len)
        .collect::<Vec<isize>>();

    let p2_offset = code[0..7]
        .iter()
        .map(|c| c.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap() as usize;

    println!("p2 offset: {:?}", p2_offset);
    println!("code length is {}", code.len());

    let code_len = code.len();

    let phases = 100;
    for phase in 1..=phases {
        let mut prev_total = 0;
        code = (0..code_len)
            .map(|nth| {
                let prod_sum = if prev_total == 0 {
                    // calc first by brute force
                    brute_calc(code_len, nth, seq_len, &code, &sequence)
                } else {
                    // calc others as diff to first
                    let delta = smart_calc(code_len, nth, seq_len, &code, &sequence);
                    prev_total + delta
                };

                prev_total = prod_sum;
                prod_sum.abs() % 10
            })
            .collect();
        println!("phase {} done", phase);
        if phase == phases {
            let result = code.iter().take(8).collect::<Vec<&isize>>();
            println!("> {} {:?}", phase, result);
            println!("p2: {:?}", &code[p2_offset..p2_offset + 8]);
        }
    }
}
