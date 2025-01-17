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

    let mut running = true;
    let mut board = BoardState::starting_state();
    let log_path = Path::new("log.txt");
    let mut comm = Comm::create(log_path).unwrap();
    board::print_bitboard(0x1000000000);
    // while running {}
}
