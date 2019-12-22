use std::collections::HashMap;

type Coordinate = (isize, isize);

#[derive(Debug, PartialEq)]
pub enum Offset {
    FRONT,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    WALL,
    WALKABLE,
    OXYGEN,
    FOG,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn translate_coordinates(pair: Coordinate, d: &Direction) -> Coordinate {
    let (mut x, mut y) = pair;
    match d {
        Direction::NORTH => y -= 1,
        Direction::SOUTH => y += 1,
        Direction::EAST => x += 1,
        Direction::WEST => x -= 1,
    };
    (x, y)
}

struct Actor {
    pub initial_position: Coordinate,
    x: isize,
    y: isize,
}

impl Actor {
    fn new(x: isize, y: isize) -> Actor {
        Actor {
            initial_position: (x, y),
            x,
            y,
        }
    }
    fn coord_pair(&self) -> Coordinate {
        (self.x, self.y)
    }
    fn move_to_dir(&mut self, d: &Direction) {
        let (x, y) = translate_coordinates(self.coord_pair(), d);
        self.x = x;
        self.y = y;
    }
}

pub struct Map {
    actor: Actor,
    pub map: HashMap<Coordinate, Tile>,
    flow: HashMap<Coordinate, (usize, Direction, Tile)>,
    target: Coordinate,
}

impl<'a> Map {
    pub fn new(actor_x: isize, actor_y: isize) -> Map {
        let mut map = Map {
            actor: Actor::new(actor_x, actor_y),
            map: HashMap::new(),
            flow: HashMap::new(),
            target: (0, 0),
        };
        map.mark_tile(&Tile::WALKABLE, map.actor.coord_pair());
        map
    }

    pub fn mark_tile(&mut self, tile: &Tile, coords: Coordinate) {
        self.map.insert(coords, tile.clone());
        // this may panic if our map sides are too small
        /*println!(
            "marking {} {} as {:?}, was {:?}",
            x,
            y,
            tile.clone(),
            self.map[index as usize]
        );*/
    }

    fn get_tile(&self, coords: &Coordinate) -> Option<&Tile> {
        // this may panic if our map sides are too small
        self.map.get(coords)
    }

    pub fn explore_fog(&mut self) -> Option<Direction> {
        let actor_pos = self.actor.coord_pair();
        // if we have discovered that target is a wall
        let target_accessible = self.get_tile(&self.target).unwrap() != &Tile::WALL;
        println!("targe {:?} accessible {}", self.target, target_accessible);
        println!("{:?}", self.flow.get(&self.target));

        let (new_flow, direction) = match (target_accessible, &self.flow.get(&actor_pos)) {
            (true, Some((distance, direction, _))) if *distance > 0usize => {
                // we have not arrived at our destination yet
                // respond with direction in flow response to reach our target
                println!("moving to {:?}", self.target);
                (None, Some(direction.clone()))
            }
            _ => {
                // we have arrived
                // find all accessible tiles
                let flow = FlowMap::access_target(self, actor_pos);
                // pick next fog to explore
                let mut first_fog = None;
                for (k, (_, _, tile)) in flow {
                    if tile == Tile::FOG {
                        first_fog = Some(k);
                        break;
                    }
                }
                match first_fog {
                    Some(fog) => {
                        self.target = fog;
                        let flow = FlowMap::access_target(self, first_fog.unwrap());
                        let (_, dir, _) = flow.get(&actor_pos).expect("actor pos shoud be flowed");

                        (Some(flow.clone()), Some(dir.clone()))
                    }
                    None => (None, None),
                }
            }
        };

        if let Some(f) = new_flow {
            self.flow = f;
        }

        direction
    }

    pub fn explored(
        &mut self,
        move_happened: bool,
        move_dir: &Direction,
        new_tile: &Tile,
        tile_offset: &Option<Offset>,
    ) {
        if move_happened {
            self.actor.move_to_dir(move_dir);
        }

        let new_tile_coord = if let Some(offset) = tile_offset {
            match offset {
                Offset::FRONT => translate_coordinates(self.actor.coord_pair(), move_dir),
            }
        } else {
            (self.actor.x, self.actor.y)
        };

        self.mark_tile(new_tile, new_tile_coord);
    }
}

pub struct FlowMap<'a> {
    pub map: &'a Map,
    pub target: Coordinate,
    pub flow: HashMap<Coordinate, (usize, Direction, Tile)>,
}

impl<'a> FlowMap<'a> {
    fn new(map: &Map, target: Coordinate) -> FlowMap {
        FlowMap {
            map,
            target,
            flow: HashMap::new(),
        }
    }

    pub fn access_target(
        map: &Map,
        target: Coordinate,
    ) -> HashMap<Coordinate, (usize, Direction, Tile)> {
        let mut flow_map = FlowMap::new(map, target);
        let distance = 0;
        flow_map.target = target;
        flow_map._access_here(target, Direction::NORTH, distance, true);
        flow_map.flow
    }

    fn _access_here(
        &mut self,
        pos: Coordinate,
        came_from: Direction,
        distance: usize,
        force: bool,
    ) {
        //println!("setting entry {:?} {:?}", pos, self.map.get_tile(&pos));
        let (entry_distance, entry_from, entry_tile) = self.flow.entry(pos).or_insert((
            distance,
            came_from.clone(),
            self.map.get_tile(&pos).unwrap().clone(),
        ));

        /*println!(
            "flow accessing {:?} {} from:{:?} {:?}",
            pos, entry_distance, entry_from, entry_tile
        );*/

        if distance == 0 || distance <= *entry_distance {
            *entry_distance = distance;
            *entry_from = came_from;
        } else {
            // if we are hitting blocks that are closer than us, then
            // we are on the wrong track and should back out
            return;
        }

        // do not clip deep into walls and fog, just one edge layer
        if *entry_tile == Tile::WALL || (*entry_tile == Tile::FOG && !force) {
            return;
        }

        self._access_here(
            translate_coordinates(pos, &Direction::NORTH),
            Direction::SOUTH,
            distance + 1,
            false,
        );
        self._access_here(
            translate_coordinates(pos, &Direction::SOUTH),
            Direction::NORTH,
            distance + 1,
            false,
        );
        self._access_here(
            translate_coordinates(pos, &Direction::EAST),
            Direction::WEST,
            distance + 1,
            false,
        );
        self._access_here(
            translate_coordinates(pos, &Direction::WEST),
            Direction::EAST,
            distance + 1,
            false,
        );
    }
}
