use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PortalType {
    INNER,
    OUTER,
}

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
    let lines: Vec<&str> = contents.split('\n').collect();
    lines.iter().map(|l| l.to_string()).collect()
}

struct FlowMap {
    pub map: Vec<String>,
    pub target: Loc,
    pub portals: HashMap<Loc, (Loc, PortalType)>,
    pub passables: Vec<char>,
    pub flows: Vec<HashMap<Loc, (usize, Dir, char, HashSet<char>)>>,
}

impl FlowMap {
    fn new() {
        panic!("dont instanciate");
    }
    pub fn generate(
        map: Vec<String>,
        portals: &HashMap<Loc, (Loc, PortalType)>,
        target: Loc,
        passables: &Vec<char>,
    ) -> HashMap<Loc, (usize, Dir, char, HashSet<char>)> {
        let depth_flows: Vec<HashMap<_, _>> = (0..30).map(|_| HashMap::new()).collect();

        let mut flow_map = FlowMap {
            map,
            passables: passables.clone(),
            portals: portals.clone(),
            target: target.clone(),
            flows: depth_flows,
        };

        let distance = 0;
        let depth = 0;
        flow_map._access_here(target, Dir::NORTH, distance, depth, HashSet::new());
        for (i, thing) in flow_map.flows.iter().enumerate() {
            println!("depth {} as {} marked", i, thing.len());
        }
        flow_map.flows.first().unwrap().clone()
    }

    fn _access_here(
        &mut self,
        pos: Loc,
        came_from: Dir,
        distance: usize,
        depth: usize,
        mut passed: HashSet<char>,
    ) {
        // before checking for '.', see if we are in portal
        let portal_at = self.portals.get(&Loc { x: pos.x, y: pos.y });

        // handle portals
        let (pos, depth) = if let Some((portal, portal_type)) = portal_at {
            let res = match portal_type {
                // ignore outer portals in 0 depth
                PortalType::OUTER => {
                    if depth == 0 {
                        (pos.clone(), depth)
                    } else {
                        (portal.clone(), depth - 1)
                    }
                }
                PortalType::INNER => (portal.clone(), depth + 1),
            };
            println!(
                "teleport from {:?} to {:?} new depth {} distance {:?}",
                pos, res.0, res.1, distance
            );
            res
        } else {
            (pos, depth)
        };

        // if we are too deep, turn back
        if depth > self.flows.len() - 1 {
            return;
        }

        let row = self.map[pos.y].as_bytes();
        let col = row[pos.x] as char;

        // do not clip deep into walls and fog, just one edge layer
        if col != '.' {
            return;
        }

        let l = Loc { x: 13, y: 2 };
        if pos == l {
            //println!("> {} {}", depth, distance);
        }

        let (entry_distance, entry_from, entry_tile, _) = self.flows[depth]
            .entry(pos.clone())
            .or_insert((distance, came_from.clone(), col, passed.clone()));

        if distance == 0 || distance <= *entry_distance {
            *entry_distance = distance;
            *entry_from = came_from.clone();
        } else {
            // if we are hitting blocks that are closer than us, then
            // we are on the wrong track and should back out
            return;
        }

        let _tmp = entry_tile.clone();
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
            depth,
            passed.clone(),
        );
        self._access_here(
            Loc {
                x: pos.x,
                y: pos.y + 1,
            },
            Dir::NORTH,
            distance + 1,
            depth,
            passed.clone(),
        );
        self._access_here(
            Loc {
                x: pos.x + 1,
                y: pos.y,
            },
            Dir::WEST,
            distance + 1,
            depth,
            passed.clone(),
        );
        self._access_here(
            Loc {
                x: pos.x - 1,
                y: pos.y,
            },
            Dir::EAST,
            distance + 1,
            depth,
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
        portals: &HashMap<Loc, (Loc, PortalType)>,
        passables: &Vec<char>,
    ) -> HashMap<Loc, (usize, Dir, char, HashSet<char>)> {
        FlowMap::generate(self.data.clone(), portals, target.clone(), passables)
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

    fn get_tile(&self, loc: &Loc) -> char {
        let raw = &self.data[loc.y].as_bytes();
        raw[loc.x] as char
    }

    fn get_tile_portal(&self, loc: &Loc) -> Option<(Loc, PortalType, Vec<char>)> {
        if loc.x < 2 || loc.y < 2 {
            return None;
        }
        if loc.x + 2 >= self.data[loc.y].len() {
            return None;
        }
        if loc.y + 2 >= self.data.len() {
            return None;
        }

        if self.get_tile(&loc) != '.' {
            return None;
        }

        let x = loc.x;
        let y = loc.y;
        let portal_type =
            match x == 2 || x >= self.data[loc.y].len() - 5 || y == 2 || y >= self.data.len() - 5 {
                true => PortalType::OUTER,
                false => PortalType::INNER,
            };

        let left = self.get_tile(&Loc {
            x: loc.x - 1,
            y: loc.y,
        });
        if left.is_alphabetic() {
            return Some((
                Loc {
                    x: loc.x - 1,
                    y: loc.y,
                },
                portal_type,
                vec![
                    self.get_tile(&Loc {
                        x: loc.x - 2,
                        y: loc.y,
                    }),
                    left,
                ],
            ));
        }

        let right = self.get_tile(&Loc {
            x: loc.x + 1,
            y: loc.y,
        });
        if right.is_alphabetic() {
            return Some((
                Loc {
                    x: loc.x + 1,
                    y: loc.y,
                },
                portal_type,
                vec![
                    right,
                    self.get_tile(&Loc {
                        x: loc.x + 2,
                        y: loc.y,
                    }),
                ],
            ));
        }

        let top = self.get_tile(&Loc {
            x: loc.x,
            y: loc.y - 1,
        });
        if top.is_alphabetic() {
            return Some((
                Loc {
                    x: loc.x,
                    y: loc.y - 1,
                },
                portal_type,
                vec![
                    self.get_tile(&Loc {
                        x: loc.x,
                        y: loc.y - 2,
                    }),
                    top,
                ],
            ));
        }

        let bottom = self.get_tile(&Loc {
            x: loc.x,
            y: loc.y + 1,
        });
        if bottom.is_alphabetic() {
            return Some((
                Loc {
                    x: loc.x,
                    y: loc.y + 1,
                },
                portal_type,
                vec![
                    bottom,
                    self.get_tile(&Loc {
                        x: loc.x,
                        y: loc.y + 2,
                    }),
                ],
            ));
        }

        None
    }

    fn find_named(&self, item: &[char]) -> Option<Loc> {
        for (y, line) in self.data.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                let loc = Loc { x, y };
                if let Some((_, _, name)) = self.get_tile_portal(&loc) {
                    if name == item {
                        return Some(loc);
                    }
                }
            }
        }
        None
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

    fn portal_mapping(&self) -> HashMap<Loc, (Loc, PortalType)> {
        let mut locations: HashMap<Vec<char>, Vec<(Loc, Loc, PortalType)>> = HashMap::new();

        for (y, line) in self.data.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                //returns loc of neighboring portal with name
                let loc = Loc { x, y };
                let maybe_portal = self.get_tile_portal(&loc);
                match maybe_portal {
                    None => continue,
                    Some((portal, portal_type, name)) => {
                        let mut data = locations.entry(name.clone()).or_insert(vec![]);
                        println!("found label {:?} @ {:?} to {:?}", name, loc, portal);
                        data.push((portal, loc, portal_type));
                    }
                }
            }
        }

        let mut output = HashMap::new();
        for (portal_name, portal_locations) in locations {
            if portal_locations.len() < 2 {
                println!("skipping label {:?}", portal_name);
                continue;
            }
            println!("binding portal {:?} => {:?}", portal_name, portal_locations);
            let (first_portal, first_landing, first_type) = portal_locations.get(0).unwrap();
            let (second_portal, second_landing, second_type) = portal_locations.get(1).unwrap();
            output.insert(
                first_portal.clone(),
                (second_landing.clone(), first_type.clone()),
            );
            output.insert(
                second_portal.clone(),
                (first_landing.clone(), second_type.clone()),
            );
        }
        output
    }
}

fn main() {
    let passables = vec!['.'];

    let map = Mapsor { data: load_map() };
    map.print();

    let actor = map.find_named(&['A', 'A']).unwrap();
    let target = map.find_named(&['Z', 'Z']).unwrap();
    let portals = map.portal_mapping();
    println!("actor {:?} target {:?}", actor, target);

    let actor_flow = map.item_flows(&actor, &portals, &passables);
    println!("dist {:?} = {:?}", &target, actor_flow.get(&target));
}
