// Check winner
pub fn check(player_move_int: u8, cpu_move_int: u8) {
    let player_win: &str = "You won!";
    let cpu_win: &str = "CPU won!";
    let draw: &str = "It is a draw!";

    if player_move_int == cpu_move_int {
        println!("{}", draw);
    } else if player_move_int == 0 {
        if cpu_move_int == 1 {
            println!("{}", cpu_win);
        } else if cpu_move_int == 2 {
            println!("{}", player_win);
        }
    } else if player_move_int == 1 {
        if cpu_move_int == 0 {
            println!("{}", player_win);
        } else if cpu_move_int == 2 {
            println!("{}", cpu_win);
        }
    } else if player_move_int == 2 {
        if cpu_move_int == 0 {
            println!("{}", cpu_win);
        } else if cpu_move_int == 1 {
            println!("{}", player_win);
        }
    }
}
