mod board;
mod comm;
use crate::board::*;
use crate::comm::*;
use std::path::Path;

fn main() {
    let starting_fen_string = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
    let board = BoardState::state_from_fen(starting_fen_string.to_owned());
    // println!("{:?}", board);
    // board.unwrap().print_board();
    // let mut engine_input = String::new();
    // let log_path = Path::new("log.txt");
    // let mut comm = Comm::create(log_path).unwrap();
    // if !comm.prelude() {
    //     return;
    // }
    // while engine_input != "quit" {
    //     engine_input = comm.engine_in();
    // }
}
