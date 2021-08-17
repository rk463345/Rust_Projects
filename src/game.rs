use rand::Rng;

fn play(rounds: u32, bot: String) -> (){
    let legal_choices = ("r", "p", "S");
    let p1_win_conditions = ("rs", "sp", "pr");

    let (win_1, win_2, win_3) = p1_win_conditions;

    if bot == "y\n" || bot == "\n" {
        let mut rounds_left = rounds;
        loop{
            let mut bot_choice = legal_choices[rand::thread_rng().gen_range(0..2)];

            let mut player_choice = String::new();
            println!("What is your choice");
            let player_choice = player_choice.trim();

            let round = player_choice + bot_choice;

            match round{
                win_1 => {println!("Player 1 wins!");},
                win_2 => {println!("Player 1 wins!");},
                win_3 => {println!("Player 1 wins!");},
                _     => {println!("Bot wins!");}
            }
            rounds_left -= 1;
            if rounds_left == 0{
                break;
            }
        }

    }
}
