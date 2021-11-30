use rand::thread_rng;
use rand::Rng;

mod help;
mod check_win;

fn main() {
    //print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to sps!
    \nFor the help page insert \"h\" at any time.");

    // Main game loop
    loop {
        let player_move_int: u8 = player_move();
        let cpu_move_int: u8 = cpu_move();

        check_win::check(player_move_int, cpu_move_int);

        break
    }
}

fn player_move() -> u8 {
    let mut player_move;

    loop {
        // Player move
        println!("Player move:");
        player_move = String::new();
        std::io::stdin()
            .read_line(&mut player_move)
            .unwrap();

        if !check_move_validity(&player_move) {
            // Check if an option was called
            // if true return to start of loop
            if player_move.trim() == "h" {
                help::manual();
                continue
            } else {
                // If no option was called declare invalid and
                // return to start of loop
                //print!("\x1B[2J\x1B[1;1H");
                println!("Invalid move");
                continue
            }
        }
        break
    };

    let player_move_int: u8 = player_move_to_int(&player_move);

    player_move_int
}

fn cpu_move() -> u8 {
    // Generate random number in 0..3 range to pick random move
    let mut rng = thread_rng();
    let cpu_move_int: u8 = rng.gen_range(0..3);

    cpu_move_int
}

//TODO: Combine with player_move_to_int() and return as tuple?
fn check_move_validity(player_move: &str) -> bool {
    // Check if player_move is valid
    // if valid return true
    // if invalid return false
    let is_valid: bool = match player_move.trim() {
        "rock" | "r" | "stone" => true,
        "paper" | "p" => true,
        "scissors" | "s" => true,
        _ => false
    };

    is_valid
}

fn player_move_to_int(player_move: &str) -> u8 {
    // Return u8 refering to move made by player
     let player_move_int = match player_move.trim() {
        "rock" | "r" | "stone" => 0,
        "paper" | "p" => 1,
        "scissors" | "s" => 2,
        _ => panic!()
     };

     player_move_int
}
