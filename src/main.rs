use std::process;
use guessing_game::guessing_game;

mod guessing_game;

fn main() {
    println!("\
             This is a basic guessing game. The ramdon rumber will be \
             generated in the backend and you need to try to guess it.\
             ");

    match guessing_game() {
       Ok(_) => {
           println!("Game Stoped!");
           process::exit(0);
       },
       Err(err) => {
            eprintln!("Error: {}", err);
           process::exit(1);
       }
    }
}

