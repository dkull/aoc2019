pub fn load_map() -> Vec<Vec<char>> {
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
    tokens
}

fn tile_is_bug(map: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    if x < 0 || x >= map[0].len() as isize || y < 0 || y >= map.len() as isize {
        return false;
    }
    map[y as usize][x as usize] == '#'
}

fn update_map(old_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn compute_score(map: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile == &'#' {
                score += 1 << ((y * line.len()) + x);
            }
        }
    }
    score
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
}

fn main() {
    let mut map = load_map();
    print_map(&map);
    println!("score: {}", compute_score(&map));

    let mut seen_scores = vec![];
    for _ in 0..1 << (5 * 5) {
        seen_scores.push(false);
    }
    //let mut seen_scores = [false; 1 << (5 * 5)].to_vec();

    for epoch in 1..=1 << (5 * 5) {
        //println!();
        println!("epoch {}", epoch);
        map = update_map(map);
        print_map(&map);
        let score = compute_score(&map);
        //println!("score: {}", score);
        if seen_scores[score] {
            panic!("epoch {} seen score {}", epoch, score);
        }
        seen_scores[score] = true;
    }
}
