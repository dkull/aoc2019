mod intcode;
mod map;

use intcode::{load_code, spawn_processes};
use map::{Direction, Map, Offset, Tile};

fn part1(map: &Map) -> u32 {
    let mut sum: u32 = 0;
    for (coord, tile) in &map.map {
        let top = map.map.get(&(coord.0, coord.1 - 1));
        let bot = map.map.get(&(coord.0, coord.1 + 1));
        let left = map.map.get(&(coord.0 - 1, coord.1));
        let right = map.map.get(&(coord.0 + 1, coord.1));
        let crossroads = match (Some(tile), top, bot, left, right) {
            (
                Some(Tile::WALKABLE),
                Some(Tile::WALKABLE),
                Some(Tile::WALKABLE),
                Some(Tile::WALKABLE),
                Some(Tile::WALKABLE),
            ) => true,
            _ => false,
        };
        if crossroads {
            sum += (coord.0 * coord.1) as u32;
            println!("crossroads: {:?}", coord);
        }
    }
    sum
}

fn main() {
    let code = load_code(2);
    let mut map = Map::new(0, 0);

    let proc_count = 1;
    let min_outputs = 1;
    let mut processes = spawn_processes(proc_count, code, vec![].into(), false);

    // load map
    let proc = &mut processes[0];
    let mut x = 0;
    let mut y = 0;
    loop {
        proc.run_to_interrupt(min_outputs);
        let move_result = proc.output.pop_front();
        let tile = match move_result {
            Some(10) => {
                y += 1;
                x = -1;
                println!();
                None
            }
            Some(35) => {
                print!("#");
                Some(Tile::WALKABLE)
            }
            Some(46) => {
                print!(".");
                Some(Tile::WALL)
            }
            Some(94) => {
                print!("^");
                None
            }
            Some(60) => {
                print!("<");
                None
            }
            Some(62) => {
                print!(">");
                None
            }
            Some(118) => {
                print!("v");
                None
            }
            None => break,
            o => panic!("bad output : >{:?}<", o),
        };
        if let Some(t) = tile {
            map.mark_tile(&t, (x, y));
        }
        x += 1;
    }

    println!("p1: {}", part1(&map));

    //process
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
                oo if oo == 1 => (true, move_dir, Tile::WALKABLE, None),
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
}
