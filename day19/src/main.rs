mod intcode;

use intcode::{load_code, spawn_processes};

fn run_with_coords(code: &Vec<isize>, x: isize, y: isize) -> isize {
    let proc_count = 1;
    let min_outputs = 1;
    let mut processes = spawn_processes(proc_count, code.clone(), vec![].into(), false);
    let proc = &mut processes[0];
    proc.input.push_back(x);
    proc.input.push_back(y);
    proc.run_to_interrupt(min_outputs);
    let move_result = proc.output.pop_front();
    move_result.unwrap()
}

fn main() {
    let code = load_code(2);

    for x in 0..1500 {
        for y in 0..1500 {
            let result = run_with_coords(&code, x, y);
            if result == 1 {
                println!("y starts at x {} y {}", x, y);
                let xfrom = x - 99;
                let yto = y + 99;
                if xfrom < 0 {
                    continue;
                }
                let bot_left = run_with_coords(&code, xfrom, yto);
                if bot_left == 1 {
                    println!("found {} {} => {}", xfrom, y, xfrom * 10000 + y);
                    panic!("found!");
                }
                break;
            }
        }
    }
}
