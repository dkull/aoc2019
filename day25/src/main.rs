#![feature(vec_remove_item)]
mod intcode;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
use std::{thread, time};

use intcode::{load_code, spawn_processes};

#[derive(Clone)]
enum ExplorationState {
    NoDoor,
    Unexplored,
    DeadEnd,
    Visited,
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

fn reverse_direction(dir: &String) -> String {
    match dir.as_str() {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        "" => "",
        e => panic!("bad direction {}", e),
    }
    .into()
}

struct Room {
    name: String,
    east: ExplorationState,
    west: ExplorationState,
    north: ExplorationState,
    south: ExplorationState,
    items: Vec<String>,
    directions: HashMap<String, String>,
    visited: bool,
    dead_end: bool,
    entry: String,
}

impl Room {
    fn new(name: String, entry: String) -> Room {
        Room {
            name,
            east: ExplorationState::NoDoor,
            west: ExplorationState::NoDoor,
            north: ExplorationState::NoDoor,
            south: ExplorationState::NoDoor,
            items: vec![],
            directions: HashMap::new(),
            visited: false,
            dead_end: false,
            entry,
        }
    }
    fn load(&mut self, doors: Vec<String>, items: Vec<String>) {
        for door in doors {
            match door.as_str() {
                "north" => self.north = ExplorationState::Unexplored,
                "south" => self.south = ExplorationState::Unexplored,
                "east" => self.east = ExplorationState::Unexplored,
                "west" => self.west = ExplorationState::Unexplored,
                e => panic!("bad door name {}", e),
            }
        }
        for item in items {
            self.items.push(item);
        }
    }

    fn visit_next(&mut self) -> String {
        // this is cheating, I know this order leads me to the security checkpoint last
        let data = vec![
            ("north", &self.north),
            ("west", &self.west),
            ("east", &self.east),
            ("south", &self.south),
        ];

        let unexploreds: Vec<String> = data
            .iter()
            .filter_map(|(dir, expl)| match expl {
                ExplorationState::Unexplored => Some(dir.to_string()),
                _ => None,
            })
            .collect();

        let visiteds: Vec<String> = data
            .iter()
            .filter_map(|(dir, expl)| match expl {
                ExplorationState::Visited => Some(dir.to_string()),
                _ => None,
            })
            .collect();

        let mut doors = vec![unexploreds.clone(), visiteds.clone()].concat();

        // make entry be the last door to go to
        let entry = doors.remove_item(&self.entry);
        if let Some(e) = entry {
            doors.push(e);
        }

        // mark direction as visited
        let first = doors.first().unwrap().clone();
        match first.as_str() {
            "north" => self.north = ExplorationState::DeadEnd,
            "south" => self.south = ExplorationState::DeadEnd,
            "east" => self.east = ExplorationState::DeadEnd,
            "west" => self.west = ExplorationState::DeadEnd,
            _ => panic!("bad"),
        }

        // mark self a dead end
        if doors.len() == 1 {
            self.dead_end = true;
        }

        first
    }
}

fn parse_room(lines: &String) -> (String, Vec<String>, Vec<String>) {
    let mut doors = vec![];
    let mut items = vec![];
    let mut name = String::new();
    let blacklist = vec![
        "escape pod",
        "molten lava",
        "photons",
        "infinite loop",
        "giant electromagnet",
    ];

    for line in lines.lines() {
        if line.starts_with("==") {
            name = line.into();
        }
        if line.starts_with('-') {
            let thing = &line[2..];
            if vec!["north", "south", "east", "west"].contains(&thing) {
                doors.push(thing.to_string());
            } else {
                if blacklist.contains(&thing) {
                    continue;
                }
                items.push(thing.to_string());
            }
        }
    }
    (name, doors, items)
}

fn main() {
    let code = load_code(2);

    let proc_count = 1;
    // data is sent in three packets
    let min_outputs = 1;
    let mut processes = spawn_processes(proc_count, code, vec![].into(), false);
    let proc = &mut processes[0];

    let mut rooms: HashMap<String, Room> = HashMap::new();

    let mut last_room_name: String = String::new();
    let mut current_room_name: String = String::new();

    let mut last_move = String::new();
    let mut room_lines = String::new();

    let mut collection_mode = true;

    let mut inventory = vec![];

    let mut combinations = vec![];
    let mut nth_combination = 0;

    let mut dropping = true;
    let mut dropping_nth = 0;
    let mut picking_up_nth = 0;

    loop {
        proc.run_to_interrupt(min_outputs);
        // read output
        if let Some(ch) = proc.output.pop_front() {
            room_lines.push(ch as u8 as char);
        }
        println!(">>> {}", room_lines);

        // if room output complete, do things
        if proc.needs_input() {
            if collection_mode {
                let (name, doors, items) = parse_room(&room_lines);
                if room_lines.contains("You take the") {
                    current_room_name = last_room_name.clone();
                } else {
                    current_room_name = name.clone();
                }
                if current_room_name.contains("Security Checkpoint") {
                    collection_mode = false;
                    combinations = powerset(&inventory);
                    continue;
                }
                room_lines.clear();

                // create the FIRST room
                if rooms.get(&current_room_name).is_none() {
                    let mut room =
                        Room::new(current_room_name.clone(), reverse_direction(&last_move));
                    room.load(doors, vec![]);
                    room.visited = true;
                    rooms.insert(current_room_name.clone(), room);
                }

                let room = rooms.get_mut(&current_room_name).unwrap();

                // find the next unexplored room
                if let Some(item) = items.first() {
                    inventory.push(item.clone());
                    format!("take {}", item).bytes().for_each(|c| {
                        proc.input.push_back(c as isize);
                    });
                    proc.input.push_back(10);
                    last_room_name = current_room_name;
                } else {
                    let explore_command = room.visit_next();
                    let input_command = explore_command.clone();

                    input_command.bytes().for_each(|c| {
                        proc.input.push_back(c as isize);
                    });
                    proc.input.push_back(10);

                    last_move = explore_command;
                    last_room_name = current_room_name;

                    thread::sleep(time::Duration::from_millis(20));
                }
            } else {
                //thread::sleep(time::Duration::from_millis(20));
                let sensor_room = room_lines.contains("Pressure-Sensitive Floor");
                let entry_allowed = !room_lines.contains("ejected back");
                println!(">>>{}<<<", room_lines);
                room_lines.clear();
                if sensor_room && entry_allowed {
                    panic!("done");
                }
                if dropping {
                    if dropping_nth < inventory.len() {
                        let itm = &inventory[dropping_nth];
                        dropping_nth += 1;
                        println!("drop {}", itm);
                        format!("drop {}", itm).bytes().for_each(|c| {
                            proc.input.push_back(c as isize);
                        });
                        proc.input.push_back(10);
                    } else {
                        println!("end dropping");
                        dropping_nth = 0;
                        dropping = false;
                    }
                } else {
                    println!("> {:?}", combinations[nth_combination]);
                    if picking_up_nth < combinations[nth_combination].len() {
                        let itm = &combinations[nth_combination][picking_up_nth as usize];
                        println!("pickup {}", itm);
                        picking_up_nth += 1;
                        format!("take {}", itm).bytes().for_each(|c| {
                            proc.input.push_back(c as isize);
                        });
                        proc.input.push_back(10);
                    } else {
                        picking_up_nth = 0;
                        nth_combination += 1;
                        println!("move");
                        "south".bytes().for_each(|c| {
                            proc.input.push_back(c as isize);
                        });
                        proc.input.push_back(10);
                        dropping = true;
                        println!("==============");
                    }
                }
                //println!("items {:?}", inventory);
            }
        }
    }
}
