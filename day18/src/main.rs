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
    pub flow: HashMap<Loc, (usize, Dir, char, HashSet<char>)>,
}

impl FlowMap {
    fn new() {
        panic!("dont instanciate");
    }
    pub fn generate(
        map: Vec<String>,
        target: Loc,
    ) -> HashMap<Loc, (usize, Dir, char, HashSet<char>)> {
        let mut flow_map = FlowMap {
            map,
            target: target.clone(),
            flow: HashMap::new(),
        };
        let distance = 0;
        flow_map._access_here(target, Dir::NORTH, distance, HashSet::new());
        flow_map.flow
    }

    fn _access_here(&mut self, pos: Loc, came_from: Dir, distance: usize, passed: HashSet<char>) {
        let row = self.map[pos.y].as_bytes();
        let col = row[pos.x];

        let (entry_distance, entry_from, entry_tile, passed) = self
            .flow
            .entry(pos.clone())
            .or_insert((distance, came_from.clone(), col as char, passed));

        if distance == 0 || distance <= *entry_distance {
            *entry_distance = distance;
            *entry_from = came_from;
        } else {
            // if we are hitting blocks that are closer than us, then
            // we are on the wrong track and should back out
            return;
        }

        // do not clip deep into walls and fog, just one edge layer
        if *entry_tile == '#' {
            return;
        }
        let mut passed = passed.clone();
        passed.insert(entry_tile.clone());

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

fn door_order(map: &Mapsor, actor: &Loc) -> Vec<char> {
    let flows = map.item_flows(actor, &vec!['.']);

    // find what blocks what
    let mut blockers = HashMap::new();
    for (k, v) in flows {
        let item = v.2;
        for blocked_by in v.3 {
            if blocked_by != '@' && blocked_by != '.' {
                let e = blockers.entry(blocked_by).or_insert(vec![]);
                e.push(item);
                blockers.entry(item).or_insert(vec![]);
            }
        }
    }

    // find what blocks nothing
    let mut non_blockers = vec![];
    for b in &blockers {
        println!("> {} blocks {:?}", b.0, b.1);
        if b.1.is_empty() {
            non_blockers.push(b.0);
        }
    }

    let mut block_chain: Vec<&char> = non_blockers.clone();

    let mut block_chain_groups = vec![];
    block_chain_groups.push(non_blockers.clone());

    loop {
        let mut any_blocker = false;

        println!("---");
        for (item, blocked) in &blockers {
            // keys can't block
            if item.is_ascii_lowercase() {
                continue;
            }
            let mut blocks = false;
            for b in blocked {
                if non_blockers.contains(&b) {
                    blocks = true;
                    break;
                }
            }
            if blocks {
                let mut can_be_unblocker = true;
                for item_blocked in blocked {
                    if !item_blocked.is_ascii_uppercase() {
                        continue;
                    }
                    if !block_chain.contains(&item_blocked) {
                        can_be_unblocker = false;
                        break;
                    }
                }

                if !can_be_unblocker {
                    any_blocker = true;
                    continue;
                }

                if !block_chain.contains(&item) {
                    println!("{} is unblocked", item);
                    block_chain.push(item);
                    let mut blocked_any = false;
                    let last_group: &mut Vec<&char> = block_chain_groups.last_mut().unwrap();
                    for ch in last_group.iter() {
                        if blocked.contains(&ch.to_ascii_lowercase()) {
                            println!("item {} blocks {} key", item, ch);
                            blocked_any = true;
                            break;
                        }
                    }
                    if blocked_any {
                        block_chain_groups.push(vec![item]);
                    } else {
                        last_group.push(item);
                    }
                }
            }
        }
        if !any_blocker {
            break;
        }
    }
    println!("blockchain: {:?}", block_chain);
    println!("blockchain gorups: {:?}", block_chain_groups);

    vec![]
}

struct Mapsor {
    data: Vec<String>,
}

impl Mapsor {
    fn print(&self) {
        for l in &self.data {
            println!("{}", l);
        }
    }

    fn item_flows(
        &self,
        target: &Loc,
        passables: &Vec<char>,
    ) -> HashMap<Loc, (usize, Dir, char, HashSet<char>)> {
        let fm = FlowMap::generate(self.data.clone(), target.clone());
        let mut goods = HashMap::new();
        for (k, v) in fm {
            if v.2 == '#' || v.2 == '.' {
                continue;
            }
            println!("{:?} -> {:?}", k, v);
            goods.insert(k, v);
        }
        goods
    }

    fn find_item(&self, item: char) -> Option<Loc> {
        for (y, line) in self.data.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == item {
                    return Some(Loc { x, y });
                }
            }
        }
        None
    }
}

fn main() {
    let map_data = load_map();
    let map = Mapsor { data: map_data };
    map.print();
    let actor_loc = map.find_item('@').unwrap();
    println!("actor @ {:?}", actor_loc);
    let order = door_order(&map, &actor_loc);
}
