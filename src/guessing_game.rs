use core::num;
use std::{i32, io};
use rand::prelude::*;

pub enum Game {
  Won,
  Stop,
}

fn gen_random_number(bottom: i32, top: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(bottom..=top)
}

fn start(bottom: i32, top: i32) -> io::Result<Game> {
    // Generate ramdon number
    let random_number: i32 = gen_random_number(bottom, top);

    loop {
        println!("What is your guess?");
        // Register user response
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Verify response
        if input == random_number {
            println!("Congrats You Won!!");
            return Ok(Game::Won);
        } else {
            // Ask if wants to continue guessing
            let mut response = String::new();
            println!("Do you want to try again?(yes, no):");
            io::stdin().read_line(&mut response)?;
            if response.trim() == "no" { return Ok(Game::Stop); }
        }
    }
}

pub fn guessing_game() -> io::Result<()> {
    // Get the lowest value
    println!("Enter the lowest possible value: (e.g. 0)");
    let mut bottom = String::new();
    io::stdin().read_line(&mut bottom)?;
    let bottom: i32 = match bottom.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number.");
            return guessing_game();
        },
    };

    // Get the highest value
    println!("Enter the highest possible value: (e.g. 100)");
    let mut top = String::new();
    io::stdin().read_line(&mut top)?;
    let top: i32 = match top.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number.");
                return guessing_game();
            }
    };

    // Play the game and handle the erros
    match start(bottom, top) {
        Ok(Game::Won) => {
            let mut response = String::new();
            println!("Want to play another game? (yes, no)");
            io::stdin().read_line(&mut response)?;
            if response.trim() == "yes" { guessing_game()?; } // Recursive call
            return Ok(())
        },
        Ok(Game::Stop) => return Ok(()),
        Err(err) => Err(err),
    }
}

