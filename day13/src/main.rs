extern crate random_integer;

mod intcode;
use intcode::{load_code, spawn_processes};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    HPADDLE,
    BALL,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Screen {
    inited: HashMap<Coord, Tile>,
    input_buf: Vec<i32>,
    score: i32,
}
impl Screen {
    fn new() -> Screen {
        Screen {
            inited: HashMap::new(),
            input_buf: vec![],
            score: -1,
        }
    }
    fn process_output(&mut self, new_output: i32) -> bool {
        self.input_buf.push(new_output);
        if self.input_buf.len() == 3 {
            let (x, y) = (self.input_buf[0], self.input_buf[1]);

            if x == -1 && y == 0 {
                self.score = self.input_buf[2];
                println!("new score: {} {} left", self.score, self.p1_result());
                self.input_buf.clear();
                return true;
            }

            let tile = match self.input_buf[2] {
                0 => Tile::EMPTY,
                1 => Tile::WALL,
                2 => Tile::BLOCK,
                3 => Tile::HPADDLE,
                4 => Tile::BALL,
                _ => panic!("bad tile"),
            };

            let step = tile == Tile::BALL;
            self.draw_to(Coord { x, y }, tile);
            self.input_buf.clear();
            return step;
        }
        false
    }

    fn draw_to(&mut self, c: Coord, t: Tile) {
        self.inited.insert(c, t);
    }

    fn p1_result(&self) -> usize {
        self.inited.values().filter(|t| **t == Tile::BLOCK).count()
    }

    fn get_ball_coord(&self) -> &Coord {
        self.inited
            .iter()
            .filter(|(_, t)| **t == Tile::BALL)
            .map(|(c, _)| c)
            .collect::<Vec<&Coord>>()[0]
    }
}

fn main() {
    let mut code = load_code();

    let mut screen = Screen::new();

    let proc_count = 1;
    code[0] = 2;
    let mut processes = spawn_processes(proc_count, code, vec![].into(), false);
    let mut hack_offsets = HashSet::new();
    let mut no_reset = 0;

    loop {
        let process_state = &mut processes[0];

        process_state.run_to_interrupt();

        if process_state.is_terminated() {
            println!("TERMINATE");
            break;
        }

        let output = process_state.output;
        if screen.process_output(output as i32)
            && screen.score >= 0
            && process_state.input.is_empty()
        {
            let ball_y = screen.get_ball_coord().y;
            process_state.input.push_back(0);
            // if no offsets, find the ones that match ball coord
            if hack_offsets.is_empty() {
                for (i, mem) in process_state.memory.iter().enumerate() {
                    if *mem as i32 == ball_y {
                        hack_offsets.insert(i);
                    }
                }
            // if we have the memory offset, tweak it when needed to save the ball, or stuck
            } else if hack_offsets.len() == 1 {
                no_reset += 1;
                let new_y = random_integer::random_isize(2, 4);
                for o in &hack_offsets {
                    let offset = *o;
                    if no_reset > 1000 || process_state.memory[offset] >= 22 {
                        process_state.memory[offset] = new_y;
                        no_reset = 0;
                    }
                }
            // filter out memory offsets that aren't the correct vale any more
            } else {
                let mut remove = vec![];
                for offset in &hack_offsets {
                    let mem_value = &process_state.memory[*offset];
                    if *mem_value as i32 != ball_y {
                        remove.push(offset.clone());
                    }
                }
                for r in remove.iter() {
                    hack_offsets.remove(r);
                }
            }
        }
    }
}
