use std::io;

fn main() {
    print_intro()
}

fn print_intro() {
    println!("Welcome to TicTacToe!");
    println!("Input the number below to choose a square to place your mark.");

    play_loop();
}

type Board<'a> = [&'a str; 9];

enum BoardState {
    Win,
    Draw,
    Undecided,
}

fn play_loop() {
    let mut is_game_over = false;
    let mut board: Board = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut player_turn = "X";

    while !is_game_over {
        println!("");
        println!("");
        display_game_board(board);
        let mut is_user_input_valid = false;

        while !is_user_input_valid {
            let player_input = get_user_input(player_turn);

            let result = update_board(&mut board, player_turn, player_input);
            match result {
                Ok(()) => is_user_input_valid = true,
                Err(_err) => {
                    println!("Square has already been taken. Please try again.");
                }
            }
        }

        let board_state = check_win_condition(board, player_turn);

        match board_state {
            BoardState::Win => {
                display_game_board(board);
                println!("Player {} has won!", player_turn);
                is_game_over = true;
            }
            BoardState::Undecided => match player_turn {
                "X" => player_turn = "O",
                "O" => player_turn = "X",
                _ => println!("Game is broken?!? Congratulations I guess"),
            },
            BoardState::Draw => {
                display_game_board(board);
                println!("Game is a draw. Congratulations to both players.");
                is_game_over = true;
            }
        }
    }
}

fn display_game_board(board: Board) {
    for (i, square) in board.iter().enumerate() {
        let square_as_int = i + 1;

        if square_as_int % 3 != 0 {
            print!(" {} |", square);
        } else if square_as_int == 9 {
            print!(" {}\n", square);
        } else {
            print!(" {}\n", square);
            println!("___|___|___")
        }
    }
}

fn get_user_input(player: &str) -> u8 {
    println!("Player {}, select a square.", player);
    let mut did_input_fail = false;
    let mut input_as_int: u8 = 0;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let input_as_int_result = input.trim().parse::<u8>();
    match input_as_int_result {
        Ok(good_input) => {
            if good_input > 0 && good_input < 10 {
                input_as_int = good_input;
            } else {
                did_input_fail = true;
            }
        }
        Err(_bad) => did_input_fail = true,
    }

    while did_input_fail {
        println!("Invalid input please try again:");
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        let input_as_int_result = input.parse::<u8>();
        match input_as_int_result {
            Ok(good_input) => {
                did_input_fail = false;
                input_as_int = good_input;
            }
            Err(_bad) => did_input_fail = true,
        }
    }

    return input_as_int;
}

enum UpdateBoardError {
    SquareAlreadyOccupied,
}

fn update_board<'a>(
    board: &mut Board<'a>,
    player_turn: &'a str,
    player_selection: u8,
) -> Result<(), UpdateBoardError> {
    let index_to_check = (player_selection - 1) as usize;
    let selected_sqaure = board[index_to_check];

    if selected_sqaure == "X" || selected_sqaure == "O" {
        return Err(UpdateBoardError::SquareAlreadyOccupied);
    } else {
        board[index_to_check] = player_turn;
        return Ok(());
    }
}

fn check_win_condition(board: Board, player: &str) -> BoardState {
    let mut did_win = false;

    let win_possibilities = [
        [board[0], board[1], board[2]],
        [board[3], board[4], board[5]],
        [board[6], board[7], board[8]],
        [board[0], board[3], board[6]],
        [board[1], board[4], board[7]],
        [board[2], board[5], board[8]],
        [board[0], board[4], board[8]],
        [board[6], board[4], board[2]],
    ];

    for win_line in win_possibilities {
        if calculate_is_slice_won(win_line, player) == true {
            did_win = true;
        }
    }

    if did_win == true {
        return BoardState::Win;
    }

    if is_board_filled(board) {
        return BoardState::Draw;
    }

    BoardState::Undecided
}

fn is_board_filled(board: Board) -> bool {
    let mut is_board_filled = true;

    for spot in board {
        if spot == "X" || spot == "O" {
            continue;
        } else {
            is_board_filled = false;
            break;
        }
    }

    is_board_filled
}

fn calculate_is_slice_won(board_slice: [&str; 3], player: &str) -> bool {
    let mut is_slice_won = true;

    for spot in board_slice {
        if spot != player {
            is_slice_won = false;
            break;
        }
    }

    is_slice_won
}
