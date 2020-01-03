use std::collections::HashMap;

pub fn load_map() -> [[char; 5]; 5] {
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

    let mut map = [['.'; 5]; 5];
    for (y, line) in tokens.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            map[y][x] = *tile;
        }
    }
    map
}

fn tile_is_bug(map: &[[char; 5]; 5], x: isize, y: isize) -> bool {
    if x < 0 || x >= map[0].len() as isize || y < 0 || y >= map.len() as isize {
        return false;
    }
    map[y as usize][x as usize] == '#'
}

fn update_map(maps: &HashMap<isize, [[char; 5]; 5]>, layer: &isize) -> [[char; 5]; 5] {
    let old_map = maps.get(&layer).unwrap();
    let mut new_map = old_map.clone();

    for (y, line) in old_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let adjacent_count: usize = vec![
                tile_is_bug(&old_map, x as isize - 1, y as isize),
                tile_is_bug(&old_map, x as isize + 1, y as isize),
                tile_is_bug(&old_map, x as isize, y as isize - 1),
                tile_is_bug(&old_map, x as isize, y as isize + 1),
            ]
            .iter()
            .filter(|t| t == &&true)
            .count();

            if tile == &'#' && adjacent_count != 1 {
                new_map[y][x] = '.';
            } else if tile == &'.' && (adjacent_count == 1 || adjacent_count == 2) {
                new_map[y][x] = '#';
            }
        }
    }
    new_map
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
}

fn get_clear_map() -> [[char; 5]; 5] {
    [['.'; 5]; 5]
}

fn main() {
    let mut maps = HashMap::new();

    // initialize the first layers
    maps.insert(-1, get_clear_map());
    maps.insert(0, load_map());
    maps.insert(1, get_clear_map());

    for epoch in 1..=200 {
        println!("epoch {}", epoch);
        let mut new_maps = HashMap::new();
        for layer in maps.keys() {
            let map = update_map(&maps, layer);
            new_maps.insert(*layer, map);
        }
        for (k, v) in new_maps {
            maps.insert(k, v);
        }
    }
}
