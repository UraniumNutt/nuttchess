mod board;
mod comm;
use crate::board::*;
use crate::comm::*;
use std::path::Path;

fn main() {
    // let starting_fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let mut board = BoardState::state_from_fen(starting_fen_string.to_owned().split(' ')).unwrap();
    // println!("{:?}", board);
    // board.print_board();

    // board.apply_move("d2d4").unwrap();
    // println!("{:?}", board);
    // board.print_board();

    let mut engine_input = String::new();
    let log_path = Path::new("log.txt");
    let mut comm = Comm::create(log_path).unwrap();
    let mut board: BoardState;
    if !comm.prelude() {
        return;
    }
    let mut fen_string = String::new();
    while engine_input != "quit" {
        engine_input = comm.engine_in();

        let mut split_input = engine_input.split(" ");
        if split_input.next().unwrap() == "position" {
            if split_input.next().unwrap() == "fen" {
                board = BoardState::state_from_fen(split_input).unwrap();

                comm.engine_out(format!("info string {:?}", board));
            }
        }
    }
}
