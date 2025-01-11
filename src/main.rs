mod board;
mod comm;
use crate::board::*;
use crate::comm::*;
use rand::seq::SliceRandom;
use std::path::Path;

fn main() {
    // let starting_fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let mut board = BoardState::state_from_fen(starting_fen_string.to_owned().split(' ')).unwrap();
    // println!("{:?}", board);
    // board.print_board();

    // board.apply_move("d2d4").unwrap();
    // println!("{:?}", board);
    // board.print_board();
    // board.apply_move("g8f6").unwrap();
    // println!("{:?}", board);
    // board.print_board();

    let mut engine_input;
    let mut running = true;
    let mut pos_set = false;
    let mut board = BoardState::starting_state();
    let log_path = Path::new("log.txt");
    let mut comm = Comm::create(log_path).unwrap();
    while running {
        engine_input = comm.engine_in();

        let mut tokens = engine_input.split(" ");

        match tokens.next().unwrap() {
            // quit
            "quit" => {
                running = false;
            }
            // uci
            "uci" => {
                comm.engine_out("id name nuttchess".to_string());
                comm.engine_out("id author UraniumNutt".to_string());
                comm.engine_out("uciok".to_string());
            }
            // isready
            "isready" => {
                comm.engine_out("readyok".to_string());
            }
            // ucinewgame
            "ucinewgame" => {}
            // position
            "position" => match tokens.next().unwrap() {
                "fen" => {
                    board = match BoardState::state_from_fen(&mut tokens) {
                        Ok(e) => e,
                        Err(e) => {
                            comm.engine_out(format!("Invalid fen string {}", e.to_string()));
                            continue;
                        }
                    };

                    match tokens.next().unwrap() {
                        "moves" => {
                            while let Some(chess_move) = tokens.next() {
                                if let Err(e) = board.apply_string_move(chess_move) {
                                    comm.engine_out(format!(
                                        "Error applying move: {}",
                                        e.to_string()
                                    ));
                                }
                            }
                            pos_set = true;
                            // print for testing
                            // board.print_board();
                        }
                        e => comm.engine_out(format!("Unexpected token {}", e)),
                    }
                }
                "startpos" => match tokens.next().unwrap() {
                    "moves" => {
                        board = BoardState::starting_state();
                        while let Some(chess_move) = tokens.next() {
                            if let Err(e) = board.apply_string_move(chess_move) {
                                comm.engine_out(format!("Error applying move: {}", e.to_string()));
                            }
                        }
                        pos_set = true;
                        // print for testing
                        // board.print_board();
                    }
                    e => comm.engine_out(format!("Unexpected token {}", e)),
                },
                e => comm.engine_out(format!("Unexpected token {}", e)),
            },
            "go" => {
                if !pos_set {
                    comm.engine_out("No position set".to_string());
                    continue;
                }

                // TODO make this not temporary
                // comm.engine_out("bestmove d7d5".to_string());
                let moves = board.generate_moves();
                let randmove = moves.choose(&mut rand::thread_rng()).unwrap();
                let move_string = randmove.to_string().unwrap();
                comm.engine_out(format!("bestmove {}", move_string));
            }
            // If the option is not recognized
            e => {
                comm.engine_out(format!("Unknown option {}", e));
            }
        }
    }
}
