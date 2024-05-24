use std::{collections::HashMap, io};

const VALID_PROFESSIONS: [&str; 3] = ["F", "A", "R"];

fn main() {
    let mut incentive_rate: HashMap<String, f32> = HashMap::new();

    incentive_rate.insert("F".to_string(), 0.5);
    incentive_rate.insert("A".to_string(), 0.35);
    incentive_rate.insert("R".to_string(), 0.25);

    println!("Choose your profession");
    println!("- F for Full Teacher");
    println!("- A for Assistant Teacher");
    println!("- R for Researcher");

    let mut profession = String::new();
    io::stdin()
        .read_line(&mut profession)
        .expect("failed reading profession");
    let profession: String = profession.trim().to_uppercase();

    validate_profession(&profession);

    println!("Insert your gross salary: ");
    let mut salary = String::new();
    io::stdin()
        .read_line(&mut salary)
        .expect("failed reading salary");
    let salary: f64 = salary
        .trim()
        .parse()
        .expect("failed to converting input salary");

    println!("Insert the total spent with equipments:");
    let mut incentive = String::new();
    io::stdin()
        .read_line(&mut incentive)
        .expect("failed to reading incentive");
    let incentive: f64 = incentive
        .trim()
        .parse()
        .expect("incentive may is not a number");

    let rate = incentive_rate
        .get(&profession)
        .expect("There is not incentive rate for this profession")
        .clone();

    let max_incentive = rate as f64 * salary;

    if incentive > max_incentive {
        println!("Incentive denied");
    };

    if incentive <= max_incentive {
        println!("Incentive allowed");
    }
}

fn validate_profession(profession: &String) {
    for valid_profession in VALID_PROFESSIONS {
        if valid_profession == profession.trim() {
            return;
        }
    }
    panic!("Invalid profession error")
}
