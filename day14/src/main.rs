use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn parse_file() -> Vec<Chemical> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.trim().split('\n').collect::<Vec<&str>>();

    let mut chemicals: HashMap<String, Chemical> = HashMap::new();

    let mut parsed_lines = vec![];
    let mut i = 0;
    'nextline: while lines.len() > parsed_lines.len() {
        i %= lines.len();
        let line = lines[i];
        let thisline = i;
        i += 1;
        if parsed_lines.contains(&thisline) {
            continue;
        }

        let mut recipe = vec![];
        let parts = line.split(" => ").collect::<Vec<&str>>();

        // parse input
        let inputs = parts[0].split(", ").collect::<Vec<&str>>();

        let mut ore_cost = 0;
        for input in inputs.iter() {
            let input_parts = input.split(' ').collect::<Vec<&str>>();
            let (input_count, input_name) =
                (input_parts[0].parse::<u64>().unwrap(), input_parts[1]);
            if input_name == "ORE" {
                ore_cost = input_count;
            } else {
                if chemicals.get(input_name).is_none() {
                    continue 'nextline;
                }
                let input_chemical = chemicals.get(input_name).unwrap();
                recipe.push((input_chemical.clone(), input_count));
            }
        }

        //parse output
        let output = parts[1].split(' ').collect::<Vec<&str>>();
        let (output_count, output_name) = (output[0].parse::<u64>().unwrap(), output[1]);

        let chemical = Chemical::new(&output_name, output_count, recipe, ore_cost);
        parsed_lines.push(thisline);
        chemicals.insert(chemical.name.clone(), chemical.clone());
    }

    chemicals.into_iter().map(|(_, v)| v).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
    batch_size: u64,
    batch_ore_cost: u64,
    inputs: Vec<(Chemical, u64)>,
}

impl Chemical {
    pub fn new(
        name: &str,
        batch_size: u64,
        recipe: Vec<(Chemical, u64)>,
        ore_cost: u64,
    ) -> Chemical {
        Chemical {
            name: name.into(),
            batch_size,
            inputs: recipe,
            batch_ore_cost: ore_cost,
        }
    }
    fn build_chain(
        &self,
        count: u64,
        mut surplus: HashMap<Chemical, u64>,
    ) -> (u64, HashMap<Chemical, u64>) {
        let mut cost = 0;
        let mut built = 0;
        let surplus_count = surplus.entry(self.clone()).or_insert(0);

        if self.batch_ore_cost > 0 {
            if *surplus_count >= count {
                *surplus_count -= count;
            } else {
                let (ore_cost, surplus) = self.get_ore_cost_for(count - *surplus_count);
                cost += ore_cost;
                *surplus_count = surplus;
            }
        } else {
            // we have built some into surplus, use that
            let count = if *surplus_count > count {
                *surplus_count -= count;
                0
            } else {
                let res = count - *surplus_count;
                *surplus_count = 0;
                res
            };

            let (batch_count, batch_surplus) = self.batch_production_info(count);
            for (input_chem, input_count) in self.inputs.iter() {
                let input_count = *input_count * batch_count;
                let result = input_chem.build_chain(input_count, surplus);
                cost += result.0;
                surplus = result.1;
            }
            built += self.batch_size * batch_count;

            let surplus_count = surplus.entry(self.clone()).or_insert(0);
            if built > count {
                *surplus_count = built - count;
            }
        }

        (cost, surplus)
    }

    pub fn get_ore_cost_for(&self, count: u64) -> (u64, u64) {
        let mut cost = 0;
        let mut produced = 0;

        while produced < count {
            produced += self.batch_size;
            cost += self.batch_ore_cost;
        }
        (cost, produced - count)
    }

    fn batch_production_info(&self, need_chems: u64) -> (u64, u64) {
        let mut produced = 0;
        let mut batches = 0;

        while produced < need_chems {
            produced += self.batch_size;
            batches += 1;
        }
        (batches, produced - need_chems)
    }
}

fn part1(chemicals: Vec<Chemical>) {
    for chem in chemicals.iter() {
        if chem.name != "FUEL" {
            continue;
        }
        let surplus = HashMap::new();
        let (cost, surplus) = chem.build_chain(1, surplus);
        println!("part1: {}", cost);
    }
}

fn part2(chemicals: Vec<Chemical>) {
    fn test(chemicals: Vec<Chemical>, number: u64) -> u64 {
        let mut result = 0;
        for chem in chemicals.iter() {
            if chem.name != "FUEL" {
                continue;
            }
            let surplus = HashMap::new();
            let (cost, surplus) = chem.build_chain(number, surplus);
            result = cost;
        }
        result
    }

    let target = 10u64.pow(12);
    let mut bot = 0;
    let mut top = 10u64.pow(7);
    loop {
        let test_val = (bot + top) / 2;
        println!("p2 testing: {} < {} < {}", bot, test_val, top);
        let result = test(chemicals.clone(), test_val);
        println!(
            "part2 candidate {}: cost {} ({:.5} of trill)",
            test_val,
            result,
            result as f64 / target as f64,
        );

        if result > target {
            top = test_val;
        } else if result < target {
            if top == bot + 1 {
                println!("match found: {}", bot);
                break;
            }
            bot = test_val;
        } else {
            println!("umm, seems to be exactly {} {}", test_val, result);
            break;
        }
    }
}

fn main() {
    let chemicals = parse_file();
    part1(chemicals.clone());
    part2(chemicals.clone());
}
