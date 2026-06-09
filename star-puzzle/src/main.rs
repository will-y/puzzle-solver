use std::io;
use std::io::Write;
use star_puzzle::board::Board;

fn main() {
    println!("Starting");
    play_game();
    // let mut board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();
    //
    // println!("Board:");
    // board.print();
    //
    // println!("Solved:");
    // board.place_star(0, 2);
    // board.place_star(1, 4);
    // board.place_star(2, 0);
    // board.place_star(3, 3);
    // board.place_star(4, 1);
    // board.print();
}

fn play_game() {
    let mut board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();

    loop {
        board.print();
        print!(">");
        io::stdout().flush().unwrap();

        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read move");

        input_str.retain(|c| !c.is_whitespace());

        let mut dot = false;

        if input_str.starts_with("D") || input_str.starts_with("d") {
            dot = true;
            input_str.remove(0);
        }

        let split: Vec<&str> = input_str.split(",").collect();

        if split.len() == 2 {
            if dot {
                board.place_dot(split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap());
            } else {
                board.place_star(split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap());
            }

            if board.is_solved() {
                println!("You won!");
                board.print();
                break;
            }
        } else {
            println!("Invalid input");
        }
    }
}
