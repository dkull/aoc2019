use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_mass_fuel(mass: i64, with_extra_fuel: bool) -> i64 {
	let fuel = ((mass as f64 / 3.0) - 2.0) as i64;
	if !with_extra_fuel {
		println!("mass of {} needs {} fuel", mass, fuel);
		fuel
	} else {
		if fuel <= 0 {
			return 0;	
		} else {
			return fuel + calculate_mass_fuel(fuel, true);
		}
	}
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let sum = reader.lines().fold((0, 0), |sum, line|  {
		let mass: i64 = line.unwrap().parse().expect("not a number");
		let fuel_p1 = calculate_mass_fuel(mass, false);
		let fuel_p2 = calculate_mass_fuel(mass, true);
		(sum.0 + fuel_p1, sum.1 + fuel_p2)
    });
    println!("p1: {} p2: {}", sum.0, sum.1);
}
