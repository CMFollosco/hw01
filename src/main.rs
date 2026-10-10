use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io;
const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;

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

fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}

fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;

    for i in 0..count {
        total += roll_die(seed + i);
    }

    total
}

fn print_status(gold: u64, food: u64, day: u64) {
    println!("Gold: {}", gold);
    println!("Food: {}", food);
    println!("Day: {}", day);
}

fn main() {
    println!("Enter a seed:");

    let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();

        let choice = read_number(1, 5);

        if choice == 1 {
            if !can_afford(gold, FOOD_PRICE) {
                println!("get more gold.");
                continue;
            }

            let max_food = gold / FOOD_PRICE;

            println!("how many units of food?");

            let amount = read_number(1, max_food);

            gold -= amount * FOOD_PRICE;
            food += amount;
            day += 1;
        } else if choice == 2 {
            let roll = roll_die(seed + roll_count);

            roll_count += 1;

            let gained = hunt_food(roll);

            food += gained;

            println!("rolled {} and gained {} food", roll, gained);

            day += 1;
        } else if choice == 3 {
            let total = roll_many(seed + roll_count, 3);

            roll_count += 3;

            let profit = total * 2;

            gold += profit;

            println!("caravan trade earned {} gold", profit);

            day += 1;
        } else if choice == 4 {
            print_status(gold, food, day);
        } else {
            println!("final stats:");
            print_status(gold, food, day);
            break;
        }
    }
}
