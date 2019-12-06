use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn find_hops(a: Vec<String>, b: Vec<String>) -> u32 {
    let mut a_distance = 0;
    let mut b_distance = 0;
    for aa in a {
        // FIXME: lookups in Vec are slow
        if b.contains(&aa) {
            println!("common {}", aa); 
            for bb in b {
                if bb == aa {
                    break;
                }
                b_distance += 1;
            }
            break
        }
        a_distance += 1;
    }
    a_distance + b_distance
}

fn count_indirect(planet: &str, orbits: &HashMap<String, String>) -> Vec<String> {
    let orbiting = orbits.get(planet).unwrap();
    if orbiting == "COM" {
        return vec!["COM".to_string()];
    }
    let mut path = count_indirect(orbiting, orbits);
    path.push(orbiting.to_string());
    path
}

fn run(orbits: HashMap<String, String>, interests: HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut orbits_sum = 0;
    let mut interesting_orbits: HashMap<String, Vec<String>> = HashMap::new();

    for (moon, _) in &orbits {
        let mut path = count_indirect(&moon, &orbits);
        let orbit_len = path.len();
        println!("{} has {} orbits", moon, orbit_len);
        orbits_sum += orbit_len;
        if interests.contains(moon) {
            path.reverse();
            interesting_orbits.insert(moon.into(), path); 
        }
    }
    println!("total {} orbits", orbits_sum);
    interesting_orbits
}

fn main() -> std::io::Result<()>{
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut orbits = HashMap::new();

    contents
        .trim()
        .split("\n")
        .for_each(|c| {
            let tokens: Vec<&str> = c.split(")").collect();
            let (planet, moon) = (tokens[0], tokens[1]);
            orbits.insert(moon.to_string().clone(), planet.to_string().clone());
        });

    let interests: HashSet<String> = ["YOU".to_string(), "SAN".to_string()].iter().cloned().collect();
    let interesting_orbits = run(orbits, interests);

    let distance = find_hops(
        interesting_orbits.get("YOU").unwrap().to_vec(),
        interesting_orbits.get("SAN").unwrap().to_vec()
    );

    println!("dist> {}", distance);

    Ok(())
}
