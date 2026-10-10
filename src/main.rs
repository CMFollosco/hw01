use std::io;

fn print_menu() {
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}

fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}

fn hunt_food(roll: u64) -> u64 {
    if roll == 1 {
        0
    } else if roll <= 3 {
        10
    } else if roll <= 5 {
        20
    } else {
        40
    }
}

fn read_number(min: u64, max: u64) -> u64 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let input = input.trim();

        match input.parse::<u64>() {
            Ok(number) if number >= min && number <= max => {
                return number;
            }
            _ => {
                println!("enter number from {} to {}.", min, max);
            }
        }
    }
}

fn main() {
    println!("Enter a number from 1 to 10:");
    let number = read_number(1, 10);

    println!("You entered {}", number);
}

