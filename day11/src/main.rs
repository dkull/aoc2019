mod intcode;

use intcode::{load_code, spawn_processes};
use std::collections::HashMap;

#[derive(Hash)]
enum Color {
    HULL,
    BLACK,
    WHITE,
}

#[derive(Hash, PartialEq, Eq)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Hash)]
enum Facing {
    N,
    S,
    E,
    W,
}
#[derive(Hash)]
enum NextOutput {
    PAINT,
    MOVEMENT,
}

struct Robot {
    x: isize,
    y: isize,
    facing: Facing,
    next_output_state: NextOutput,
    painted: HashMap<Coordinate, Color>,
}
impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            facing: Facing::N,
            next_output_state: NextOutput::PAINT,
            painted: HashMap::new(),
        }
    }
    fn get_coordinate(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() -> std::io::Result<()> {
    let tokens = load_code();

    let mut robot = Robot::new();
    robot.painted.insert(robot.get_coordinate(), Color::WHITE);

    let proc_count = 1;
    let mut processes = spawn_processes(proc_count, tokens, vec![1].into());

    loop {
        let process_state = &mut processes[0];
        process_state.run_to_interrupt();

        if process_state.is_terminated() {
            println!("TERMINATE");
            println!("{} splotches painted", robot.painted.len());
            break;
        }

        let output = process_state.output;

        match robot.next_output_state {
            NextOutput::PAINT => {
                let paint = match output {
                    0 => Color::BLACK,
                    1 => Color::WHITE,
                    _ => panic!("bad color"),
                };
                robot.painted.insert(robot.get_coordinate(), paint);
                robot.next_output_state = NextOutput::MOVEMENT;
            }
            NextOutput::MOVEMENT => {
                match output {
                    0 => {
                        robot.facing = match robot.facing {
                            Facing::N => Facing::W,
                            Facing::S => Facing::E,
                            Facing::E => Facing::N,
                            Facing::W => Facing::S,
                        };
                    }
                    1 => {
                        robot.facing = match robot.facing {
                            Facing::N => Facing::E,
                            Facing::S => Facing::W,
                            Facing::E => Facing::S,
                            Facing::W => Facing::N,
                        };
                    }
                    _ => panic!("bad direction"),
                };
                match robot.facing {
                    Facing::N => robot.y -= 1,
                    Facing::S => robot.y += 1,
                    Facing::E => robot.x += 1,
                    Facing::W => robot.x -= 1,
                };

                let color = match robot.painted.get(&robot.get_coordinate()) {
                    Some(c) => match c {
                        Color::BLACK => 0,
                        Color::WHITE => 1,
                        Color::HULL => 0,
                    },
                    None => 0,
                };
                process_state.input.push_back(color);
                robot.next_output_state = NextOutput::PAINT;
            }
        }
    }

    let (min_x, min_y, max_x, max_y) = {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
        for (coord, color) in robot.painted.iter() {
            min_x = std::cmp::min(min_x, coord.x);
            max_x = std::cmp::max(max_x, coord.x);

            min_y = std::cmp::min(min_y, coord.y);
            max_y = std::cmp::max(max_y, coord.y);
        }
        (min_x, min_y, max_x, max_y)
    };

    println!("{} {} {} {}", min_x, min_y, max_x, max_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(c) = robot.painted.get(&Coordinate { x, y }) {
                match c {
                    Color::WHITE => print!("#"),
                    _ => print!(" "),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}
