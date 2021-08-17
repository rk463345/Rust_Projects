use std::io;
mod game;

fn main() {
    println!(
"***************************************************
********************* WELCOME *********************
***************************************************
***************************************************
****************** Ryan\'s RPS Game ****************
***************************************************\n\n\n");

    let mut best_of = Int:new();
    let mut two_player = String:new();

    println!("Would you like to play against a bot? (y/n)");
    io::stdin()
        .read_line(&mut two_player)
        .expect("Failed to read line");

    println!("How many games would you like to play?");
    io::stdin()
        .read_line(&mut best_of)
        .expect("Failed to read line");

    let best_of: u32 = best_of.trim().parse().expect("Please enter a number");

    game::play(best_of, two_player);
}
