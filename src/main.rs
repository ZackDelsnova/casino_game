use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{Write, Read};

fn save_coins(coins: i32) -> std::io::Result<()> {
    let mut file = File::create("coins.txt")?;
    write!(file, "{}", coins)?;
    Ok(())
}

fn load_coins() -> i32 {
    let mut file = match File::open("coins.txt") {
        Ok(f) => f,
        Err(_) => return 100,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
        contents.trim().parse().unwrap_or(100)
    } else {
        100
    }

}

fn main() {
    let mut coins = load_coins();
    let mut rng = rand::thread_rng();

    while coins > 0 {
        println!("You have {} coins", coins);

        let mut bet: i32;
        loop {
            println!("Enter bet amount: ");
            bet = read_number();
            if bet > coins {
                println!("Cant Afford that BET!! Broke ASS acoount has: {} coins", coins);
            } else if bet <= 0 {
                println!("Enter a valid amount!! IDIOT");
            } else {
                break;
            }
        }

        let current = rng.gen_range(0..=100);
        println!("Current number: {}", current);
        
        println!("Will the next number be higher or lower? (h/l): ");
        let choice = read_line_trimmed().to_lowercase();

        let next = rng.gen_range(0..=100);

        if (choice == "h" && next > current) || (choice == "l" && next < current) {
            println!("Correct you win {} coins!", bet);
            println!("Next number was: {}", next);
            coins += bet;
        } else {
            println!("WRONG!!!");
            println!("Next number was: {}", next);
            println!("You lose {} coins", bet);
            coins -= bet;
        }

        println!("Play again? (y/n): ");
        let opti = read_line_trimmed();
        if opti.trim().to_lowercase() == "y" {
            continue;
        } else {
            println!("Bye Bitch");

            // Save back to file
            if let Err(e) = save_coins(coins) {
                eprintln!("Failed to save coins: {}", e);
            } else {
                println!("Coins saved.");
            }
            break;
        }
    }    

}

// Helpers

// number input
fn read_number() -> i32 {
    loop {
        let input = read_line_trimmed();
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("TYPE A NUMBER"),
        }
    }
}

// get single line and trim whitespace
fn read_line_trimmed() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

