mod intcode;

extern crate rand;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

use std::cmp::{max, min};

use intcode::{load_code, spawn_processes};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut code = load_code(2);

    let proc_count = 1;
    let min_outputs = 1;

    /*let commands = "NOT B J
    AND C J
    NOT C T
    AND D T
    OR T J
    NOT J T
    AND D T
    AND E T
    OR T J
    OR I T
    NOT T T
    OR T J
    RUN\n";
    */

    let mut known_cases: HashMap<String, usize> = [
        ("#####.###########".to_string(), 1), // 14900
        ("#####.##.########".to_string(), 2), // 35400
        ("#####.###.#######".to_string(), 3),
        ("#####...#########".to_string(), 4), // 24500
        ("#####..#.########".to_string(), 5), // 48100
        ("#####.#..########".to_string(), 6), // 42000
        ("#####.##.##...###".to_string(), 7), // 68000
        ("#####.#.##...####".to_string(), 8),
        ("#####...##...####".to_string(), 9),  // 72200
        ("#####..#..#######".to_string(), 10), // 45600
        ("#####.#..##..####".to_string(), 11), // 54400
        ("#####.####.##.###".to_string(), 12),
        ("#####.#.#.##..###".to_string(), 13), // 185000
        ("#####.####.#.####".to_string(), 14), // 92110
        ("#####...###.#.###".to_string(), 15), // 59700
        ("#####..#...#.####".to_string(), 16), // 68400
        ("#####.#.####..###".to_string(), 17), // 72490
        ("#####..#..##..###".to_string(), 18), // 58400
        ("#####.#.#########".to_string(), 19), // 33100
        ("#####.#...#.#.###".to_string(), 20), // 91738
        ("#####.##...#..###".to_string(), 21), // 86600
        ("#####..#.#.#.####".to_string(), 22), // 112126
        ("#####.#####...###".to_string(), 23), // 120000
        ("#####...#..##.###".to_string(), 24), // 129715
        ("#####.###.###.###".to_string(), 25), // 117085
        ("#####...####..###".to_string(), 26), // 102664
        ("#####...#...#.###".to_string(), 27), // 126706
        ("#####..###.#..###".to_string(), 28), // 258245
        ("#####.###.#..####".to_string(), 29), // 194691
        ("#####...#..######".to_string(), 30), // 68485
        ("#####..###.##.###".to_string(), 31), // 162230
        ("#####...####.####".to_string(), 32), // 143769
        ("#####.#..###.####".to_string(), 33), // 78542
        ("#####...#...#####".to_string(), 34), // 125901
        ("#####.#.###..####".to_string(), 35), // 211204
        ("#####.###...#####".to_string(), 36), // 150087
        ("#####.#...#######".to_string(), 37), // 125027
    ]
    .iter()
    .cloned()
    .collect();

    // found 12
    let mut best_cmds = vec![
        "NOT B J".to_string(),
        "AND C J".to_string(),
        "NOT C T".to_string(),
        "AND D T".to_string(),
        "OR T J".to_string(),
        "NOT J T".to_string(),
        "AND D T".to_string(),
        "AND E T".to_string(),
        "OR T J".to_string(),
        "OR I T".to_string(),
        "NOT T T".to_string(),
        "OR T J".to_string(),
    ];

    // found 13 - 185k
    let mut best_cmds = vec![
        "OR T J".to_string(),
        "NOT J T".to_string(),
        "AND E T".to_string(),
        "OR I T".to_string(),
        "NOT T T".to_string(),
        "NOT C T".to_string(),
        "OR T J".to_string(),
        "OR E J".to_string(),
        "OR A T".to_string(),
        "OR F J".to_string(),
        "OR F T".to_string(),
        "NOT A J".to_string(),
        "NOT A J".to_string(),
        "OR H J".to_string(),
        "AND D J".to_string(),
    ];

    /*
    // found 15
    let mut best_cmds = vec![
        "NOT A J".to_string(),
        "NOT J T".to_string(),
        "OR A T".to_string(),
        "NOT C T".to_string(),
        "AND B J".to_string(),
        "OR G J".to_string(),
        "OR C J".to_string(),
        "OR C T".to_string(),
        "AND H J".to_string(),
        "NOT J T".to_string(),
        "AND G T".to_string(),
        "NOT J T".to_string(),
        "AND D J".to_string(),
    ];

    // found 16?
    let mut best_cmds = vec![
        "OR T J".to_string(),
        "NOT B J".to_string(),
        "OR I T".to_string(),
        "AND H T".to_string(),
        "OR H J".to_string(),
        "NOT H T".to_string(),
        "AND E T".to_string(),
        "AND J J".to_string(),
        "OR H J".to_string(),
        "AND D J".to_string(),
        "OR J T".to_string(),
        "OR T J".to_string(),
    ];

    // found 17
    let mut best_cmds = vec![
        "AND H T".to_string(),
        "OR I T".to_string(),
        "NOT B J".to_string(),
        "OR H J".to_string(),
        "AND J J".to_string(),
        "OR H J".to_string(),
        "AND E T".to_string(),
        "OR T J".to_string(),
        "OR J T".to_string(),
        "NOT E T".to_string(),
        "NOT A T".to_string(),
        "AND D J".to_string(),
    ];

    // found 18
    let mut best_cmds = vec![
        "OR H J".to_string(),
        "AND J J".to_string(),
        "AND H T".to_string(),
        "OR I T".to_string(),
        "OR D J".to_string(),
        "OR C J".to_string(),
        "OR A J".to_string(),
        "AND I T".to_string(),
        "OR B T".to_string(),
        "AND H T".to_string(),
        "OR T T".to_string(),
        "AND T J".to_string(),
        "NOT B T".to_string(),
        "NOT I T".to_string(),
        "AND D J".to_string(),
    ];

    // found 19
    let mut best_cmds = vec![
        "AND F J".to_string(),
        "OR B T".to_string(),
        "OR B T".to_string(),
        "AND C J".to_string(),
        "OR D J".to_string(),
        "OR F J".to_string(),
        "OR F T".to_string(),
        "NOT B J".to_string(),
        "OR A J".to_string(),
        "OR F T".to_string(),
        "OR H T".to_string(),
        "OR C J".to_string(),
        "AND H J".to_string(),
        "AND D J".to_string(),
    ];

    // found 20
    let mut best_cmds = vec![
        "AND E T".to_string(),
        "AND J J".to_string(),
        "NOT B J".to_string(),
        "OR J T".to_string(),
        "OR I J".to_string(),
        "OR G J".to_string(),
        "AND H J".to_string(),
        "NOT E T".to_string(),
        "NOT G T".to_string(),
        "OR E J".to_string(),
        "AND T T".to_string(),
        "AND D J".to_string(),
    ];*/

    // found 22
    /*let mut best_cmds = vec![
        "OR J J".to_string(),
        "OR H J".to_string(),
        "OR T T".to_string(),
        "OR E J".to_string(),
        "NOT J T".to_string(),
        "NOT C J".to_string(),
        "AND H J".to_string(),
        "AND B J".to_string(),
        "AND F J".to_string(),
        "NOT T J".to_string(),
        "NOT I T".to_string(),
        "NOT I T".to_string(),
        "AND H T".to_string(),
        "AND D J".to_string(),
        "OR T J".to_string(),
    ];

    // found 21
    let mut best_cmds = vec![
        "NOT J T".to_string(),
        "NOT I T".to_string(),
        "OR T T".to_string(),
        "NOT C J".to_string(),
        "AND B J".to_string(),
        "NOT T J".to_string(),
        "AND H T".to_string(),
        "AND F J".to_string(),
        "OR H J".to_string(),
        "AND D J".to_string(),
        "AND H J".to_string(),
        "NOT I T".to_string(),
        "OR E J".to_string(),
        "OR D T".to_string(),
        "AND T J".to_string(),
    ];
    */

    // found 23 - 120k
    /*let mut best_cmds = vec![
        "OR H J".to_string(),
        "OR J T".to_string(),
        "OR J T".to_string(),
        "NOT H T".to_string(),
        "OR E J".to_string(),
        "AND E T".to_string(),
        "OR T J".to_string(),
        "OR A T".to_string(),
        "AND I T".to_string(),
        "OR H T".to_string(),
        "OR G T".to_string(),
        "AND T J".to_string(),
        "AND D J".to_string(),
    ];*/

    // FAIL: "#####.#.#@##..###" 187010
    // FAIL: "#####.#.#@##..###" 187661
    // FAIL: "#####.#.#.##@.###" 191216
    let mut best_cmds = vec![
        "OR F J".to_string(),
        "OR E J".to_string(),
        "OR I J".to_string(),
        "OR H J".to_string(),
        "OR F J".to_string(),
        "OR H J".to_string(),
        "OR J J".to_string(),
        "OR E J".to_string(),
        "OR I J".to_string(),
        "OR G J".to_string(),
        "NOT B J".to_string(),
        "OR H J".to_string(),
        "AND D J".to_string(),
        "NOT A T".to_string(),
        "OR T J".to_string(),
    ];

    // unseen: "#####..###@#..###" 264k
    let mut best_cmds = vec![
        "OR F J".to_string(),
        "OR H J".to_string(),
        "OR E J".to_string(),
        "OR F J".to_string(),
        "OR E J".to_string(),
        "OR F J".to_string(),
        "OR E J".to_string(),
        "OR G J".to_string(),
        "NOT C J".to_string(),
        "OR I J".to_string(),
        "OR E J".to_string(),
        "AND H J".to_string(),
        "OR J J".to_string(),
        "OR E J".to_string(),
        "AND D J".to_string(),
    ];

    // works like this too
    let mut best_cmds = vec![];

    let mut rng = rand::thread_rng();
    fn generate_command(myrng: &mut ThreadRng) -> String {
        let instructions = ["AND", "OR", "NOT"];
        let readable_regs = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "T"];
        let writable_regs = ["J", "T"];
        let mut ring = myrng;
        let instruction = instructions.choose(&mut ring).unwrap();
        let readable = readable_regs.choose(&mut ring).unwrap();
        let writable = writable_regs.choose(&mut ring).unwrap();

        return format!("{} {} {}", instruction, readable, writable);
    }

    fn cleanup(data: Vec<String>) -> Vec<String> {
        let mut data = data;

        let mut last_j = 0;
        for (i, com) in data.iter().enumerate().rev() {
            if com.ends_with('J') {
                last_j = i;
            }
        }
        if last_j > 0 {
            let popped = data.remove(last_j);
            data.push(popped);
        }

        /*let mut removed = vec![];
        for (i, com) in data.iter().enumerate().rev() {
            if com.ends_with('T') {
                removed.push(i);
            }
            if com.contains("T J") {
                break;
            }
        }
        for rem in removed {
            data.remove(rem);
        }*/
        data.dedup();
        data
    }

    fn mogrify(data: &mut Vec<String>, myrng: &mut ThreadRng) {
        enum Action {
            INSERT,
            REMOVE,
            REPLACE,
            MODIFY,
            SWAP,
        }
        let mut ring = myrng;
        let data_len = data.len();

        let actions = if data_len == 0 {
            vec![Action::INSERT]
        } else if data_len < 15 {
            vec![
                Action::INSERT,
                //Action::REMOVE,
                Action::REPLACE,
                Action::MODIFY,
                Action::SWAP,
            ]
        } else {
            vec![
                //Action::REMOVE,
                Action::REPLACE,
                Action::MODIFY,
                Action::SWAP,
            ]
        };
        let instruction = actions.choose(&mut ring).unwrap();
        let pos = ring.gen::<usize>() % max(1, data_len);

        match instruction {
            Action::INSERT => {
                data.insert(pos, generate_command(&mut ring));
            }
            Action::REMOVE => {
                data.remove(pos);
            }
            Action::REPLACE => {
                data.remove(pos);
                data.insert(pos, generate_command(&mut ring));
            }
            Action::MODIFY => {
                let donor = generate_command(&mut ring);
                let donor_tokens = donor.split(' ').collect::<Vec<&str>>();

                let command = data.get(pos).unwrap();
                let mut data_tokens = command.split(' ').collect::<Vec<&str>>();
                let which = ring.gen::<usize>() % 3;

                data_tokens.remove(which);
                data_tokens.insert(which, donor_tokens.get(which).unwrap());

                let new_data = data_tokens.join(" ");

                data.remove(pos);
                data.insert(pos, new_data);
            }
            Action::SWAP => {
                let a = ring.gen::<usize>() % data_len;
                let b = ring.gen::<usize>() % data_len;
                data.swap(a, b);
            }
        }
    }

    let mut best_score = 0;
    let mut best_score_found_on = 0;
    let mut cmds = best_cmds.clone();
    let tries = 61;
    let mut i = 0;

    let mut global_i = 0;
    let mut output_instructions: HashMap<String, usize> = HashMap::new();
    let mut score = 0;
    loop {
        i += 1;
        global_i += 1;

        // don't mangle the first run
        if global_i > 1 {
            mogrify(&mut cmds, &mut rng);
            // remove duplicates and unneccesary ->T
            cmds = cleanup(cmds);
        }

        let mut processes = spawn_processes(proc_count, code.clone(), vec![].into(), false);
        let mut program = String::new();
        for cmd in &cmds {
            program.push_str(&cmd);
            program.push('\n');
        }
        program.push_str("RUN\n");

        let input_data = program
            .bytes()
            .map(|b| b as u8 as char as isize)
            .collect::<Vec<isize>>();

        let proc = &mut processes[0];
        for inp in input_data.iter() {
            proc.input.push_back(*inp as isize);
        }

        let mut out_lines = String::new();
        loop {
            proc.run_to_interrupt(min_outputs);
            let move_result = proc.output.pop_front();
            match move_result {
                Some(i) => {
                    if i < 255 {
                        out_lines.push(i as u8 as char);
                    } else {
                        println!("answer : {}", i);
                        panic!();
                    }
                }
                None => break,
            };
        }

        let last_line = out_lines.split('\n').collect::<Vec<&str>>();
        let failure = last_line.get(last_line.len() - 3).unwrap();
        let anon_failure = failure.to_string().replace("@", ".");
        let inst_count = proc.get_instruction_count();
        //println!("FAIL: {:?} {}", failure, inst_count);

        // read the instructon count for last output
        let new_count = output_instructions.entry(anon_failure.clone()).or_insert(0);
        // store it
        *new_count = max(*new_count, inst_count);

        // if WIN!
        if anon_failure == "" {
            println!("win: {:?} {}", failure, inst_count);
            println!("{}", out_lines);
            panic!();
        }

        // if unseen case
        if !known_cases.contains_key(&anon_failure) {
            println!("unseen: {:?} {}", failure, inst_count);
            println!("unseen: {}", program);
            known_cases.insert(anon_failure.clone(), inst_count);
            //panic!("new case");
        }

        //
        score = inst_count;

        // give bonus points for upto 7 temp buffer used
        let t_writes = program.find("T\n");
        let t_reads = program.find(" T ");
        let bonus = match (t_writes, t_reads) {
            (Some(w), Some(r)) => r > w,
            _ => false,
        };

        if bonus {
            score += 1;
        };

        if score > best_score {
            best_score = score;
            best_cmds = cmds.clone();
            best_score_found_on = global_i;
            println!("FAIL: {:?} w inst count {}", failure, inst_count);
            println!(
                "NEW BEST SCORE: {} (bonus {}), score {}",
                score, bonus, program
            );
        }

        // print debug info rarely
        if global_i % 100 == 0 && score >= best_score - 5 {
            println!(
                "{:08} === {} with {} cmds best score {} (bonus {}) case {}",
                global_i,
                i,
                cmds.len(),
                best_score,
                bonus,
                failure,
            );
            //let mut ii = 0;
            /*for (k, v) in &output_instructions {
                println!("-- {}. {} {}", ii, k, v);
                ii += 1;
            }*/
        }

        let stuck = best_score_found_on < (global_i - 300_000);
        if stuck {
            println!("STUCK, resetting");
            best_score = 0;
            best_cmds = vec![];
            // let's mutate cmds as much as possible before reset
            i = 0;
        }

        if i == tries || out_lines.contains("15") {
            i = 0;
            cmds = best_cmds.clone();
        }
    }
}
