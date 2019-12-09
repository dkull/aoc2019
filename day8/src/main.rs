use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

const ROWS: usize = 6;
const COLUMNS: usize = 25;

fn extract_stats(stats_collector: &Vec<isize>) -> (isize, isize, isize) {
    let mut zeros = 0;
    let mut ones = 0;
    let mut twos = 0;
    for val in stats_collector.iter() {
        if *val == 0 {
            zeros += 1;
        }
        if *val == 1 {
            ones += 1;
        }
        if *val == 2 {
            twos += 1;
        }
    }
    (zeros, ones, twos)
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens: Vec<isize> = contents
        .trim()
        .to_string()
        .chars()
        .map(|c| c.to_string().parse::<isize>().unwrap())
        .collect();

    let mut image = [[3; COLUMNS]; ROWS];
    let mut best_result: (isize, isize, isize) = (0xffffffff, 0, 0);

    let mut stats_collector: Vec<isize> = vec![];
    for (i, val) in tokens.iter().enumerate() {
        let layer = i / (ROWS * COLUMNS);
        let pointer = i % (ROWS * COLUMNS);
        let row = pointer / COLUMNS;
        let column = pointer % COLUMNS;

        println!(
            "layer {} pos {} pointer {} row {} col {} -> {}",
            layer, i, pointer, row, column, val
        );

        stats_collector.push(*val);

        image[row][column] = match image[row][column] {
            0 => 0,
            1 => 1,
            2 => *val,
            3 => *val,
            _ => panic!("bad pixel value"),
        };

        if pointer == (ROWS * COLUMNS) - 1 {
            let (zeros, ones, twos) = extract_stats(&stats_collector);
            println!("last layer contained {} {} {}", zeros, ones, twos);
            stats_collector.clear();
            if zeros < best_result.0 {
                best_result = (zeros, ones, twos);
            }
        }
    }
    println!(
        "result p1: {:?} == {}",
        best_result,
        best_result.1 * best_result.2
    );

    for row in 0..ROWS {
        for col in 0..COLUMNS {
            let val = image[row][col];
            if val == 1 {
                print!("#");
            } else {
                print!(" ");
            }
            if col == COLUMNS - 1 {
                println!();
            }
        }
    }

    Ok(())
}
