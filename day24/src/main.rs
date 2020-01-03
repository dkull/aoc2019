use std::collections::HashMap;

const MAP_DIM: usize = 5;
const RECURSE_POS: (isize, isize) = (2, 2);

type Map = [[char; MAP_DIM as usize]; MAP_DIM as usize];

enum CameFrom {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

pub fn load_map() -> Map {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tokens: Vec<Vec<char>> = contents
        .trim()
        .split('\n')
        .map(|c| c.chars().collect())
        .collect();

    let mut map = get_clear_map();
    for (y, line) in tokens.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            map[y][x] = *tile;
        }
    }
    map
}

fn bug_count(
    layer: isize,
    maps: &HashMap<isize, Map>,
    x: isize,
    y: isize,
    came_from: CameFrom,
) -> isize {
    let mut neighbors = vec![];

    // recurse to upper levels
    let higher_layer = &(layer + 1);
    let lower_layer = &(layer - 1);

    // if higher layer
    if y < 0 || y >= MAP_DIM as isize || x < 0 || x >= MAP_DIM as isize {
        if y < 0 {
            neighbors
                .push(maps[higher_layer][(RECURSE_POS.1 - 1) as usize][RECURSE_POS.0 as usize]);
        }
        if y >= MAP_DIM as isize {
            neighbors
                .push(maps[higher_layer][(RECURSE_POS.1 + 1) as usize][RECURSE_POS.0 as usize]);
        }
        if x < 0 {
            neighbors
                .push(maps[higher_layer][RECURSE_POS.1 as usize][(RECURSE_POS.0 - 1) as usize]);
        }
        if x >= MAP_DIM as isize {
            neighbors
                .push(maps[higher_layer][RECURSE_POS.1 as usize][(RECURSE_POS.0 + 1) as usize]);
        }
    } else if x == RECURSE_POS.0 && y == RECURSE_POS.1 {
        // if lower layer
        let xs_and_ys: Vec<(usize, usize)> = match came_from {
            CameFrom::TOP => (0..MAP_DIM).map(|xx| (xx, 0)).collect(),
            CameFrom::BOTTOM => (0..MAP_DIM).map(|xx| (xx, MAP_DIM - 1)).collect(),
            CameFrom::LEFT => (0..MAP_DIM).map(|yy| (0, yy)).collect(),
            CameFrom::RIGHT => (0..MAP_DIM).map(|yy| (MAP_DIM - 1, yy)).collect(),
        };
        for (x, y) in xs_and_ys {
            //println!("taking from lower layer {} {} {}", lower_layer, x, y);
            neighbors.push(maps[lower_layer][y][x]);
        }
    } else {
        neighbors.push(maps[&(layer as isize)][y as usize][x as usize]);
    }

    neighbors.iter().filter(|n| n == &&'#').count() as isize
}

fn update_map(maps: &HashMap<isize, Map>, layer: isize) -> Map {
    let old_map = maps.get(&layer).unwrap();
    let mut new_map = *old_map;

    for (y, line) in old_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            // don't do logic for recurse tile
            if x == RECURSE_POS.0 as usize && y == RECURSE_POS.1 as usize {
                continue;
            }
            let adjacents = vec![
                bug_count(layer, &maps, x as isize - 1, y as isize, CameFrom::RIGHT),
                bug_count(layer, &maps, x as isize + 1, y as isize, CameFrom::LEFT),
                bug_count(layer, &maps, x as isize, y as isize - 1, CameFrom::BOTTOM),
                bug_count(layer, &maps, x as isize, y as isize + 1, CameFrom::TOP),
            ];
            let adjacent_count: isize = adjacents.iter().sum();

            /*println!(
                "layer {}  x:{} y:{} has {} adjacent {:?}",
                layer, x, y, adjacent_count, adjacents
            );*/

            if tile == &'#' && adjacent_count != 1 {
                new_map[y][x] = '.';
            } else if tile == &'.' && (adjacent_count == 1 || adjacent_count == 2) {
                new_map[y][x] = '#';
            }
        }
    }
    new_map
}

fn get_clear_map() -> Map {
    [['.'; MAP_DIM]; MAP_DIM]
}

fn count_bugs_in_map(map: &Map) -> isize {
    let mut bugs = 0;
    for y in map {
        for x in y {
            if x == &'#' {
                bugs += 1;
            }
        }
    }
    bugs
}

fn count_bugs(maps: &HashMap<isize, Map>) -> isize {
    let mut bugs = 0;
    maps.iter().for_each(|(_, m)| {
        bugs += count_bugs_in_map(&m);
    });
    bugs
}

fn main() {
    let mut maps: HashMap<isize, Map> = HashMap::new();
    let epochs = 200;

    // initialize the first layers
    maps.insert(0, load_map());
    for layers in 1..=epochs {
        maps.insert(-layers, get_clear_map());
        maps.insert(layers, get_clear_map());
    }

    for epoch in 1..=epochs {
        println!("start epoch {} with {} bugs", epoch, count_bugs(&maps));
        let mut new_maps = HashMap::new();
        for layer in maps.keys() {
            let possible = (maps.contains_key(&(layer - 1))
                && count_bugs_in_map(&maps[&(layer - 1)]) > 0)
                || (maps.contains_key(&(layer + 1)) && count_bugs_in_map(&maps[&(layer + 1)]) > 0)
                || count_bugs_in_map(&maps[&(layer)]) > 0;

            let map = if possible {
                update_map(&maps, *layer)
            } else {
                maps[layer]
            };

            new_maps.insert(*layer, map);
        }
        for (k, v) in new_maps {
            maps.insert(k, v);
        }
    }
    println!("bugs {}", count_bugs(&maps));
}
