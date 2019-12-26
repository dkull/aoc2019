mod intcode;
mod map;

use intcode::{load_code, spawn_processes};

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

type Coordinate = (isize, isize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Move {
    R(usize),
    L(usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    WALKABLE,
    SPACE,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Turn {
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn part1(map: HashMap<Coordinate, Tile>) -> (HashSet<Coordinate>, u32) {
    let mut crossroad_coll = HashSet::new();
    let mut sum: u32 = 0;
    for (coord, tile) in &map {
        let top = map.get(&(coord.0, coord.1 - 1));
        let bot = map.get(&(coord.0, coord.1 + 1));
        let left = map.get(&(coord.0 - 1, coord.1));
        let right = map.get(&(coord.0 + 1, coord.1));
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
            crossroad_coll.insert(coord.clone());
            sum += (coord.0 * coord.1) as u32;
            println!("crossroads: {:?}", coord);
        }
    }
    (crossroad_coll, sum)
}

fn moves_to_string(moves: &[Move]) -> String {
    let mut output = String::new();
    for mv in moves {
        let (label, dist) = match mv {
            Move::L(x) => ('L', x),
            Move::R(x) => ('R', x),
        };
        output.push(label);
        output.push(',');
        output.push_str(&format!("{},", dist));
    }
    output = output[0..output.len() - 1].to_string();
    output.push('\n');
    output
}

fn random_num(upto: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos % upto
}

fn unvisited_tiles(
    map: &HashMap<Coordinate, Tile>,
    visited: &HashSet<Coordinate>,
) -> Vec<Coordinate> {
    let mut u = vec![];
    let all_walkables = map
        .iter()
        .filter(|(_, v)| **v == Tile::WALKABLE)
        .map(|(k, _)| *k)
        .collect::<Vec<Coordinate>>();

    for walkable in all_walkables.iter() {
        if !visited.contains(walkable) {
            u.push(walkable.clone());
        }
    }

    u
}

fn new_facing(facing: Direction, maybe_turn: Option<Turn>) -> Direction {
    match maybe_turn {
        Some(turn) => match (facing, turn) {
            (Direction::NORTH, Turn::L) => Direction::WEST,
            (Direction::NORTH, Turn::R) => Direction::EAST,
            (Direction::SOUTH, Turn::L) => Direction::EAST,
            (Direction::SOUTH, Turn::R) => Direction::WEST,
            (Direction::EAST, Turn::L) => Direction::NORTH,
            (Direction::EAST, Turn::R) => Direction::SOUTH,
            (Direction::WEST, Turn::L) => Direction::SOUTH,
            (Direction::WEST, Turn::R) => Direction::NORTH,
        },
        None => facing,
    }
}

fn movement_delta(facing: Direction) -> Coordinate {
    match facing {
        Direction::NORTH => (0, -1),
        Direction::SOUTH => (0, 1),
        Direction::EAST => (1, 0),
        Direction::WEST => (-1, 0),
    }
}

fn get_tile(
    map: &HashMap<Coordinate, Tile>,
    location: &Coordinate,
    facing: Direction,
    turn: Option<Turn>,
) -> (Coordinate, Option<Tile>) {
    let new_facing = new_facing(facing, turn);
    let delta = movement_delta(new_facing);
    let lookat_location = (location.0 + delta.0, location.1 + delta.1);
    (
        lookat_location,
        match map.get(&lookat_location) {
            Some(t) => Some(t.clone()),
            None => None,
        },
    )
}

fn calc_moves_in_direction(
    map: &HashMap<Coordinate, Tile>,
    location: &Coordinate,
    facing: Direction,
    turn: Option<Turn>,
) -> Vec<Move> {
    let mut moves = vec![];
    let mut running_count = 0;

    let facing = new_facing(facing, turn);

    let mut location = location.clone();

    while let (loc, Some(Tile::WALKABLE)) = get_tile(map, &location, facing, None) {
        running_count += 1;
        let left_fork = match get_tile(map, &loc, facing, Some(Turn::L)).1 {
            Some(Tile::WALKABLE) => true,
            _ => false,
        };
        let right_fork = match get_tile(map, &loc, facing, Some(Turn::R)).1 {
            Some(Tile::WALKABLE) => true,
            _ => false,
        };
        if left_fork || right_fork {
            moves.push(match turn {
                Some(Turn::L) => Move::L(running_count),
                Some(Turn::R) => Move::R(running_count),
                None => panic!("have to turn"),
            });
        }
        location = loc.clone();
    }
    // if deadend
    if moves.is_empty() && running_count > 0 {
        moves.push(match turn {
            Some(Turn::L) => Move::L(running_count),
            Some(Turn::R) => Move::R(running_count),
            None => panic!("have to turn"),
        });
    }
    moves
}

fn calc_moves(
    map: &HashMap<Coordinate, Tile>,
    location: &Coordinate,
    facing: Direction,
) -> Vec<Move> {
    vec![
        calc_moves_in_direction(map, location, facing, Some(Turn::L)),
        calc_moves_in_direction(map, location, facing, Some(Turn::R)),
    ]
    .concat()
}

fn do_move(
    movement: &Move,
    location: Coordinate,
    facing: Direction,
) -> (Direction, Vec<Coordinate>) {
    let mut visited = vec![];

    let (turn, steps) = match movement {
        Move::L(x) => (Turn::L, x),
        Move::R(x) => (Turn::R, x),
    };

    let mut location = location;
    let facing = new_facing(facing, Some(turn));
    let delta = movement_delta(facing);

    for _ in 0..*steps as u32 {
        let new_coord = (location.0 + delta.0, location.1 + delta.1);
        location = new_coord;
        visited.push(new_coord);
    }

    (facing, visited)
}

fn subslices<T: PartialEq>(collection: &[T], sub: &[T]) -> Vec<(isize, isize)> {
    let mut matches = vec![];
    let sublen = sub.len();
    let mut i = 0;
    while i < collection.len() - sublen + 1 {
        let mut is_match = true;
        for j in 0..sublen {
            if collection[i + j] != sub[j] {
                is_match = false;
                break;
            }
        }
        if is_match {
            matches.push((i as isize, i as isize + sublen as isize));
            i += sublen;
        } else {
            i += 1;
        }
    }
    matches
}

fn find_routines(moves: Vec<Move>) -> Option<(String, String, String, String)> {
    let moves_count = moves.len() as usize;
    let min_size = 2;
    let max_size = min(moves_count, 6);

    // create subroutine A
    for a_end in min_size..=max_size {
        // contains moves in sequence covered by previous subroutines
        let a_subslice = &moves[0..a_end];
        let a_matches = subslices(&moves, a_subslice);

        'b: for b in a_end..moves_count - min_size {
            for bb in b + min_size..=moves_count {
                if bb - b > max_size {
                    continue 'b;
                }
                let b_subslice = &moves[b..bb];
                let b_matches = subslices(&moves, b_subslice);

                'c: for c in bb..moves_count - min_size {
                    for cc in c + min_size..=moves_count {
                        if cc - c > max_size {
                            continue 'c;
                        }
                        let c_subslice = &moves[c..cc];
                        let c_matches = subslices(&moves, c_subslice);

                        let mut i = 0isize;
                        let mut good = true;
                        let mut main_routine = String::new();
                        while i < moves_count as isize {
                            let is_a = a_matches
                                .iter()
                                .filter(|m| m.0 == i)
                                .collect::<Vec<&Coordinate>>();
                            let is_b = b_matches
                                .iter()
                                .filter(|m| m.0 == i)
                                .collect::<Vec<&Coordinate>>();
                            let is_c = c_matches
                                .iter()
                                .filter(|m| m.0 == i)
                                .collect::<Vec<&Coordinate>>();
                            if let Some(m) = is_a.first() {
                                i += m.1 - m.0;
                                main_routine.push_str("A,");
                                continue;
                            }
                            if let Some(m) = is_b.first() {
                                i += m.1 - m.0;
                                main_routine.push_str("B,");
                                continue;
                            }
                            if let Some(m) = is_c.first() {
                                i += m.1 - m.0;
                                main_routine.push_str("C,");
                                continue;
                            }
                            good = false;
                            break;
                        }
                        main_routine = main_routine[0..main_routine.len() - 1].to_string();
                        main_routine.push('\n');

                        if good {
                            for mv in &moves {
                                println!(">{:?}", mv);
                            }
                            return Some((
                                main_routine,
                                moves_to_string(a_subslice),
                                moves_to_string(b_subslice),
                                moves_to_string(c_subslice),
                            ));
                        }
                    }
                }
            }
        }
    }
    None
}

fn part2(
    map: HashMap<Coordinate, Tile>,
    crossroads: HashSet<Coordinate>,
    location: Coordinate,
    facing: Direction,
) -> (String, String, String, String) {
    let (mut main, mut a, mut b, mut c) =
        (String::new(), String::new(), String::new(), String::new());
    loop {
        let mut used_moves = HashSet::new();
        let mut performed_moves = vec![];
        let mut visited_tiles = HashSet::new();
        let mut location = location;
        let mut facing = facing;
        visited_tiles.insert(location);
        // assume all maps start with facing north
        loop {
            let available_moves = calc_moves(&map, &location, facing);
            // get moves that haven't been performed in this location
            let mut unseen_moves = vec![];
            for mv in available_moves {
                let (fng, dist) = match mv {
                    Move::L(x) => (new_facing(facing, Some(Turn::L)), x),
                    Move::R(x) => (new_facing(facing, Some(Turn::R)), x),
                };

                if !used_moves.contains(&(location, fng, dist)) {
                    unseen_moves.push(mv);
                }
            }

            if unseen_moves.is_empty() {
                break;
            }

            let random_move_index = random_num(unseen_moves.len() as u32) as usize;
            let chosen_move = unseen_moves[random_move_index];

            let (fng, dist) = match chosen_move {
                Move::L(x) => (new_facing(facing, Some(Turn::L)), x),
                Move::R(x) => (new_facing(facing, Some(Turn::R)), x),
            };
            used_moves.insert((location, fng, dist));

            let (new_facing, visited) = do_move(&chosen_move, location, facing);
            let new_coord = visited.last().expect("have to end up somewhere");
            if !crossroads.contains(&new_coord) && visited_tiles.contains(new_coord) {
                break;
            }

            for visit in visited.iter() {
                visited_tiles.insert(visit.clone());
            }
            performed_moves.push(chosen_move);

            location = new_coord.clone();
            facing = new_facing;
        }
        let unvisits = unvisited_tiles(&map, &visited_tiles);
        if unvisits.is_empty() {
            if performed_moves.len() > 60 {
                continue;
            }

            let found = find_routines(performed_moves);
            match found {
                None => continue,
                Some((mm, aa, bb, cc)) => {
                    println!("found {:?} {:?} {:?} {:?}", mm, aa, bb, cc);
                    main = mm;
                    a = aa;
                    b = bb;
                    c = cc;
                    break;
                }
            }
        }
    }
    (main, a, b, c)
}

fn main() {
    let mut code = load_code(2);
    let mut map = HashMap::new();

    let proc_count = 1;
    let min_outputs = 1;
    let mut processes = spawn_processes(proc_count, code.clone(), vec![].into(), false);

    // load map
    let proc = &mut processes[0];
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::NORTH;
    let mut location = (0, 0);
    loop {
        proc.run_to_interrupt(min_outputs);
        let move_result = proc.output.pop_front();
        let tile = match move_result {
            Some(10) => {
                y += 1;
                x = -1;
                None
            }
            Some(35) => Some(Tile::WALKABLE),
            Some(46) => Some(Tile::SPACE),
            Some(94) => {
                location = (x, y);
                direction = Direction::NORTH;
                Some(Tile::WALKABLE)
            }
            Some(60) => {
                location = (x, y);
                direction = Direction::WEST;
                Some(Tile::WALKABLE)
            }
            Some(62) => {
                location = (x, y);
                direction = Direction::EAST;
                Some(Tile::WALKABLE)
            }
            Some(118) => {
                location = (x, y);
                direction = Direction::SOUTH;
                Some(Tile::WALKABLE)
            }
            None => break,
            o => panic!("bad output : >{:?}<", o),
        };
        if let Some(t) = tile {
            map.insert((x, y), t.clone());
        }
        x += 1;
    }

    code[0] = 2;

    // p1
    let (crossroads, p1) = part1(map.clone());
    println!("p1: {}", p1);

    // p2
    let (main, a, b, c) = part2(map.clone(), crossroads, location, direction);
    let mut input_data: Vec<isize> = vec![];
    for ch in main.as_bytes() {
        input_data.push(*ch as isize);
    }
    for ch in a.as_bytes() {
        input_data.push(*ch as isize);
    }
    for ch in b.as_bytes() {
        input_data.push(*ch as isize);
    }
    for ch in c.as_bytes() {
        input_data.push(*ch as isize);
    }
    // 'n'
    input_data.push(110 as isize);
    input_data.push(10);

    let mut processes = spawn_processes(proc_count, code.clone(), vec![].into(), false);
    //process
    loop {
        let proc = &mut processes[0];
        for inp in input_data.iter() {
            proc.input.push_back(*inp as isize);
        }
        proc.run_to_interrupt(min_outputs);
        let move_result = proc.output.pop_front();
        println!("result: {:?}", move_result);

        if proc.is_terminated() {
            println!("TERMINATE");
            break;
        }
    }
}
