use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Dir {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Key {
    loc: Loc,
    name: char,
    flow: HashMap<Loc, (usize, Dir, char, HashSet<char>)>,
    others: Vec<char>,
}

fn load_map() -> Vec<String> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.trim().split('\n').collect();
    lines.iter().map(|l| l.to_string()).collect()
}

struct FlowMap {
    pub map: Vec<String>,
    pub target: Loc,
    pub passables: Vec<char>,
    pub flow: HashMap<Loc, (usize, Dir, char, HashSet<char>)>,
}

impl FlowMap {
    fn new() {
        panic!("dont instanciate");
    }
    pub fn generate(
        map: Vec<String>,
        target: Loc,
        passables: &Vec<char>,
    ) -> HashMap<Loc, (usize, Dir, char, HashSet<char>)> {
        let mut flow_map = FlowMap {
            map,
            passables: passables.clone(),
            target: target.clone(),
            flow: HashMap::new(),
        };

        let distance = 0;
        flow_map._access_here(target, Dir::NORTH, distance, HashSet::new());
        let pointless_items: Vec<Loc> = flow_map
            .flow
            .iter()
            .filter(|(k, v)| passables.contains(&v.2))
            .map(|(k, v)| k.clone())
            .collect();

        pointless_items.iter().for_each(|pos| {
            flow_map.flow.remove(pos);
        });
        flow_map.flow
    }

    fn _access_here(
        &mut self,
        pos: Loc,
        came_from: Dir,
        distance: usize,
        mut passed: HashSet<char>,
    ) {
        let row = self.map[pos.y].as_bytes();
        let col = row[pos.x] as char;

        // do not clip deep into walls and fog, just one edge layer
        if col == '#' {
            return;
        }

        let (entry_distance, entry_from, entry_tile, _) = self.flow.entry(pos.clone()).or_insert((
            distance,
            came_from.clone(),
            col,
            passed.clone(),
        ));

        if distance == 0 || distance <= *entry_distance {
            *entry_distance = distance;
            *entry_from = came_from.clone();
        } else {
            // if we are hitting blocks that are closer than us, then
            // we are on the wrong track and should back out
            return;
        }

        let _tmp = entry_tile.clone();
        self.flow.insert(
            pos.clone(),
            (distance, came_from.clone(), col as char, passed.clone()),
        );

        if !self.passables.contains(&_tmp) {
            passed.insert(_tmp);
        }

        self._access_here(
            Loc {
                x: pos.x,
                y: pos.y - 1,
            },
            Dir::SOUTH,
            distance + 1,
            passed.clone(),
        );
        self._access_here(
            Loc {
                x: pos.x,
                y: pos.y + 1,
            },
            Dir::NORTH,
            distance + 1,
            passed.clone(),
        );
        self._access_here(
            Loc {
                x: pos.x + 1,
                y: pos.y,
            },
            Dir::WEST,
            distance + 1,
            passed.clone(),
        );
        self._access_here(
            Loc {
                x: pos.x - 1,
                y: pos.y,
            },
            Dir::EAST,
            distance + 1,
            passed.clone(),
        );
    }
}

struct Mapsor {
    data: Vec<String>,
}

impl Mapsor {
    fn print(&self) {
        for l in &self.data {
            println!("MAP: {}", l);
        }
    }

    fn item_flows(
        &self,
        target: &Loc,
        passables: &Vec<char>,
    ) -> HashMap<Loc, (usize, Dir, char, HashSet<char>)> {
        let fm = FlowMap::generate(self.data.clone(), target.clone(), passables);
        let mut goods = HashMap::new();
        for (k, v) in fm {
            goods.insert(k, v);
        }
        goods
    }

    fn find_items(&self, item: &char) -> Vec<Loc> {
        let mut results = vec![];
        for (y, line) in self.data.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == *item {
                    results.push(Loc { x, y });
                }
            }
        }
        results
    }

    fn all_keys(&self) -> Vec<(Loc, char)> {
        let mut out = vec![];
        self.data.iter().for_each(|s| {
            s.chars().for_each(|c| {
                if c.is_ascii_lowercase() {
                    out.push((self.find_items(&c).first().unwrap().clone(), c));
                }
            })
        });
        out
    }
}

fn make_graph(map: &Mapsor, keys: Vec<(Loc, char)>, passables: &Vec<char>) -> HashMap<char, Key> {
    keys.iter()
        .map(|(pos, key)| {
            let flow = map.item_flows(&pos, &passables);
            (
                *key,
                Key {
                    loc: pos.clone(),
                    name: *key,
                    flow,
                    others: keys
                        .iter()
                        .filter(|(_, other)| other != key)
                        .map(|(_, c)| *c)
                        .collect(),
                },
            )
        })
        .collect()
}

fn solve_graph(
    robot_flows: &Vec<HashMap<Loc, (usize, Dir, char, HashSet<char>)>>,
    keys: &HashMap<char, Key>,
    used: Vec<char>,
    distances: Vec<usize>,
    bests: &mut HashMap<Vec<char>, usize>,
) {
    'next_key: for (key_name, key_item) in keys {
        if used.contains(&key_name) {
            continue;
        }

        let robot_flow = &robot_flows
            .iter()
            .filter(|f| f.iter().any(|(k, v)| v.2 == *key_name))
            .map(|f| f.clone())
            .collect::<Vec<HashMap<Loc, (usize, Dir, char, HashSet<char>)>>>();

        let robot_flow = robot_flow.first().unwrap();

        let blocked_by = &robot_flow.get(&key_item.loc.clone()).unwrap().3;

        //println!("{} blocked by {:?}", key_name, blocked_by);

        for b in blocked_by {
            // don't walk through keys
            if b.is_ascii_lowercase() && !used.contains(&b) {
                continue 'next_key;
            }
            // expect all doors to have keys picked up
            if !used.contains(&b.to_ascii_lowercase()) {
                continue 'next_key;
            }
        }

        // find all the keys taken by this robot
        let robot_lasts = used
            .iter()
            .filter(|u| {
                let key_struct = keys.get(u).unwrap();
                // get distance form previous thing to current thing
                key_struct.flow.get(&key_item.loc).is_some()
            })
            .map(|u| *u)
            .collect::<Vec<char>>();

        let distance = match robot_lasts.last() {
            None => robot_flow.get(&key_item.loc).unwrap().0,
            Some(l) => {
                let last = keys.get(l).unwrap();
                // get distance form previous thing to current thing
                last.flow.get(&key_item.loc).unwrap().0
            }
        };

        let mut distances = distances.clone();
        distances.push(distance);

        let mut used = used.clone();
        used.push(*key_name);

        let this_score = distances.iter().sum::<usize>();

        for (best, best_score) in &*bests {
            // last item has to match
            if key_name != best.last().unwrap() {
                continue;
            }
            let mut contains_me = true;
            for u in &used {
                if !best.contains(&u) {
                    contains_me = false;
                }
            }
            if contains_me && best.len() >= used.len() && best_score <= &this_score {
                continue 'next_key;
            }
        }
        bests.insert(used.clone(), this_score);
        solve_graph(robot_flows, keys, used, distances, bests);
    }
}

fn main() {
    let mut scores = HashMap::new();
    let passables = vec!['.', '@'];

    let map = Mapsor { data: load_map() };
    map.print();

    let actors = map.find_items(&'@');
    let pos_and_keys = map.all_keys();
    let actor_flows = actors
        .iter()
        .map(|a| map.item_flows(&a, &passables))
        .collect::<Vec<HashMap<Loc, (usize, Dir, char, HashSet<char>)>>>();

    let key_data = make_graph(&map, pos_and_keys.clone(), &passables);

    solve_graph(&actor_flows, &key_data, vec![], vec![], &mut scores);

    let mut best = 0xffff;
    for (k, v) in scores {
        if k.len() == pos_and_keys.len() {
            best = min(best, v);
        }
    }
    println!("best distance {}", best);
}
