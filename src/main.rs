//use regex::Regex;
use rand::Rng;
use std::env;

fn interpret_roll_statement(roll_statement: &String) -> u32 {
    //let dieRollRegex = Regex::new(r"(\d+)?d\d+").unwrap();
    //let matchedDieRolls = dieRollRegex.captures(rollStatement).unwrap();
    //println!("{:?}", matchedDieRolls);
    //rollStatement.split_off(rollStatement.find('d').expect("'d' not found."));
    let dindex: usize = roll_statement.find('d')
        .expect("malformed rollStatement: 'd' not found");
    let num_dice: u32 = roll_statement[..dindex].parse::<u32>().unwrap();
    let die_size: u32 = roll_statement[dindex+1..].parse::<u32>().unwrap();

    let mut total: u32 = 0;
    for _ in 1..num_dice {
        total += rand::thread_rng().gen_range(1, die_size);
    }
    return total;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let roll_statement: &String = &args[1];
    println!("{}", roll_statement);
    println!("{}", interpret_roll_statement(roll_statement));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3d6() {
        let result: u32 = interpret_roll_statement(&String::from("3d6"));
        assert!(result >= 3);
        assert!(result <= 18);
    }
}
