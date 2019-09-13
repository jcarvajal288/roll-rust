#[macro_use] extern crate lazy_static;

use regex::Regex;
use rand::Rng;
use std::env;

// this needs to handle all elements. result of "3d6+2d10-6" should be ["3d6", "+", "2d10", "-", "6'"]
fn extract_die_rolls(roll_statement: &String) -> Vec<String> {
    lazy_static! {
        static ref DIE_ROLL_REGEX: Regex = Regex::new(r"(\d+)?d\d+").unwrap();
    }
    //const constantRegex: Regex = Regex::new(r"[+|-]\d+\b").unwrap();
    let mut die_rolls = Vec::new();
    for die_roll in DIE_ROLL_REGEX.find_iter(roll_statement) {
        die_rolls.push(String::from(die_roll.as_str()));
    }
    return die_rolls;
}

fn interpret_roll_statement(roll_statement: &String) -> u32 {
    //let dieRollRegex = Regex::new(r"(\d+)?d\d+").unwrap();
    //let matchedDieRolls = dieRollRegex.captures(rollStatement).unwrap();
    //println!("{:?}", matchedDieRolls);
    //rollStatement.split_off(rollStatement.find('d').expect("'d' not found."));
    let dindex: usize = roll_statement.find('d')
        .expect("malformed rollStatement: 'd' not found");
    let num_dice: u32 = if dindex == 0 {
        1
    } else {
        roll_statement[..dindex].parse::<u32>().unwrap()
    };
    let die_size: u32 = roll_statement[dindex+1..].parse::<u32>().unwrap();

    let mut total: u32 = 0;
    for _ in 0..num_dice {
        total += rand::thread_rng().gen_range(1, die_size);
    }
    return total;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let roll_statement: &String = &args[1..].join("");
    println!("{}", roll_statement);
    println!("{}", interpret_roll_statement(roll_statement));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d6() {
        let result: u32 = interpret_roll_statement(&String::from("d6"));
        assert!(result >= 1);
        assert!(result <= 6);
    }

    #[test]
    fn test_3d6() {
        let result: u32 = interpret_roll_statement(&String::from("3d6"));
        assert!(result >= 3);
        assert!(result <= 18);
    }

    #[test]
    fn test_die_roll_extract() {
        let result: Vec<String> = extract_die_rolls(&String::from("3d6+2d10"));
        assert!(result.len() == 2);
        assert!(result[0] == String::from("3d6"));
        assert!(result[1] == String::from("2d10"));
    }

    #[test]
    fn test_3d6plus5() {
        let result: u32 = interpret_roll_statement(&String::from("3d6 + 5"));
        assert!(result >= 8);
        assert!(result <= 23);
    }
}
