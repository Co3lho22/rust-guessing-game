# Guessing Game
This is a simple guessing game implemented in Rust. A random number is generated, and the player needs to guess it. The game will provide feedback and continue until the correct number is guessed or the player chooses to stop.

## Purpose
This project was created as one of my first Rust projects with the goal of becoming more familiar with the Rust programming language and its error handling mechanisms.

## How to Play
1. The game will prompt you to enter the lowest and highest possible values for the random number.
2. The game will generate a random number within that range.
3. You will be asked to guess the number.
4. After each guess, the game will inform you if your guess was correct.
5. If your guess is incorrect, you will be asked if you want to try again.
6. If you choose to stop, the game ends.
7. If you guess the correct number, you win and will be asked if you want to play another game.

## Features
- Generates a random number within a user-defined range.
- Provides feedback on whether the guessed number is correct.
- Handles user input errors gracefully.
- Allows the user to stop the game at any time.
- Asks the user if they want to play another game after winning.

## Error Handling
- The game handles invalid input by prompting the user to enter a valid number.
- Uses Rust's Result type and the ? operator to manage potential I/O errors.

## Getting Started
### Prerequisites
- Rust installed on your system.
### Running the Game
1. Clone the repository:

```sh
git clone https://github.com/your-username/rust-guessing-game.git
cd rust-guessing-game
```
2. Build and run the game:

```sh
cargo run
```

## Code Overview
### Main Logic
The `main` function initializes the game and handles the main game loop, managing user input and game state.

``` rust
fn main() {
    println!("This is a basic guessing game. A random number will be generated in the backend, and you need to try to guess it.");

    match guessing_game() {
        Ok(_) => {
            println!("Game Stopped!");
            process::exit(0);
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
```

### Core Functions
- `gen_random_number(bottom: i32, top: i32) -> i32`: Generates a random number within the given range.
- `start(bottom: i32, top: i32) -> io::Result<Game>`: Handles the main guessing game logic.
- `guessing_game() -> io::Result<()>`: Manages user input for the range and initiates the game.

