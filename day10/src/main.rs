extern crate num;

use num::integer::Integer;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Debug, Eq, Hash)]
struct Vector {
    row: isize,
    col: isize,
}

impl Vector {
    fn new(row: isize, col: isize) -> Vector {
        Vector { row, col }
    }

    fn angle(&self) -> f32 {
        let deg = (self.row as f32).atan2(self.col as f32).to_degrees() + 90.0;
        if deg < 0.0 {
            (360.0 + deg).abs()
        } else {
            deg
        }
    }
}

#[derive(PartialEq, Debug)]
struct Asteroid {
    row: isize,
    col: isize,
}

impl Asteroid {
    fn new(row: isize, col: isize) -> Asteroid {
        Asteroid { row, col }
    }

    fn vector_to(&self, other: &Asteroid) -> Vector {
        let row_delta = other.row - self.row;
        let col_delta = other.col - self.col;
        let gcd = row_delta.gcd(&col_delta);
        Vector::new(row_delta / gcd, col_delta / gcd)
    }

    fn distance_to(&self, other: &Asteroid) -> f32 {
        ((self.row - other.row).abs() + (self.col - other.col).abs()) as f32
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.trim().split('\n').map(|l| l.to_string()).collect();

    let mut asteroids = vec![];

    // create all asteroids
    for (i, row) in lines.iter().enumerate() {
        for (j, col) in row.chars().enumerate() {
            if col == '#' {
                asteroids.push(Asteroid::new(i as isize, j as isize));
            }
        }
    }

    // p1
    let dummy_asteroid = Asteroid::new(0, 0);
    let (best_score, best_asteroid, best_vectors) = {
        asteroids.iter().fold(
            (0, &dummy_asteroid, HashSet::new()),
            |(best_score, best_asteroid, best_vectors), asteroid| {
                let vectors = asteroids
                    .iter()
                    .filter(|other| other != &asteroid)
                    .map(|other| asteroid.vector_to(other))
                    .collect::<HashSet<Vector>>();

                let score = vectors.len();
                if score > best_score {
                    (score, asteroid, vectors)
                } else {
                    (best_score, best_asteroid, best_vectors)
                }
            },
        )
    };

    println!("p1 best: {:?} {:?}", best_score, best_asteroid);

    // let's just bruteforce p2, making it optimal requires too much code

    // group asteroids by unit vectors to best asteroid
    let mut asteroid_groups = HashMap::new();
    asteroids
        .iter()
        .filter(|a| a != &best_asteroid)
        .for_each(|a| {
            let vector = best_asteroid.vector_to(a);
            let collection = asteroid_groups.entry(vector).or_insert(Vec::new());
            collection.push(a);
        });

    // sort the grouped asteroids by distance
    for group in asteroid_groups.values_mut() {
        group.sort_by(|a, b| {
            best_asteroid
                .distance_to(a)
                .partial_cmp(&best_asteroid.distance_to(b))
                .unwrap()
        })
    }

    // sort the vectors by degrees
    let mut best_vectors = best_vectors.iter().collect::<Vec<&Vector>>();
    best_vectors.sort_by(|a, b| a.angle().partial_cmp(&b.angle()).unwrap());
    best_vectors.iter().for_each(|a| println!("vecs {:?}", a));

    // destroy asteroids
    let mut destroyed = HashSet::new();
    'rotate_cannon: for vector in &best_vectors {
        let group = asteroid_groups.get_mut(&vector).unwrap();
        for asteroid in group.iter_mut() {
            let destroyed_key = (asteroid.row, asteroid.col);
            if !destroyed.contains(&destroyed_key) {
                destroyed.insert(destroyed_key);
                println!("destroyed nth {} asteroid {:?}", destroyed.len(), asteroid);
                continue 'rotate_cannon;
            }
        }
    }

    Ok(())
}
