mod intcode;
mod map;

use std::collections::{HashMap, HashSet};

use intcode::{load_code, spawn_processes};
use map::{Direction, FlowMap, Map, Offset, Tile};

fn main() {
    let code = load_code(2);
    let mut map = Map::new(50, 0, 0);

    let proc_count = 1;
    let min_outputs = 1;
    let mut processes = spawn_processes(proc_count, code, vec![].into(), false);

    loop {
        let proc = &mut processes[0];

        // take map and output a new movement on the map
        let move_dir = map.explore_fog();
        let new_input = match &move_dir {
            Some(Direction::NORTH) => 1,
            Some(Direction::SOUTH) => 2,
            Some(Direction::WEST) => 3,
            Some(Direction::EAST) => 4,
            None => break,
        };

        proc.input.push_back(new_input);
        proc.run_to_interrupt(min_outputs);

        let move_result = proc.output.pop_front();

        // generate task specific results
        let (move_happened, moved, new_tile, tile_offset) =
            match move_result.expect("missing output") {
                oo if oo == 0 => (false, move_dir, Tile::WALL, Some(Offset::FRONT)),
                oo if oo == 1 => (true, move_dir, Tile::EMPTY, None),
                oo if oo == 2 => (true, move_dir, Tile::OXYGEN, None),
                _ => panic!("bad output value"),
            };

        // move, and mark new tile (with offset) to map
        map.explored(move_happened, &moved.unwrap(), &new_tile, &tile_offset);

        if proc.is_terminated() {
            println!("TERMINATE");
            break;
        }
    }

    println!("p1: {}", map.part_1_distance());
    println!("p2: {}", map.part_2_steps());
}
