mod intcode;

use intcode::{load_code, spawn_processes};

fn main() {
    let mut code = load_code(2);

    let proc_count = 1;
    let min_outputs = 1;
    let mut processes = spawn_processes(proc_count, code.clone(), vec![].into(), false);

    // #!...#  == NOT A J
    // !#.##.#
    // ##.###
    //
    // !A || (A && !B  && C)
    /*let commands = "OR A T
        NOT B J
        AND T J
        AND C J
        NOT A T
        OR T J
        WALK\n";
    */
    // ##.#..##
    // NOT C && D
    let commands = "OR A T
        NOT B J
        AND T J
        AND C J
        NOT A T
        OR T J
        NOT C T
        AND D T
        OR T J
        WALK\n";
    let input_data = commands
        .bytes()
        .map(|b| b as u8 as char as isize)
        .collect::<Vec<isize>>();

    // load map
    let proc = &mut processes[0];
    for inp in input_data.iter() {
        proc.input.push_back(*inp as isize);
    }

    loop {
        proc.run_to_interrupt(min_outputs);
        let move_result = proc.output.pop_front();
        match move_result {
            Some(i) => {
                if i < 255 {
                    print!("{}", i as u8 as char);
                } else {
                    println!("answer : {}", i);
                }
            }
            None => break,
        };
    }
}
