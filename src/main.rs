mod board;
mod comm;
pub mod tables;
use crate::board::*;
use crate::comm::*;
use rand::seq::SliceRandom;
use std::path::Path;
use tables::Tables;

fn main() {
    // let starting_fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let mut board = BoardState::state_from_fen(starting_fen_string.to_owned().split(' ')).unwrap();
    // println!("{:?}", board);
    // board.print_board();
    let test = Tables::new();
    let foo = test.get_bishop_attack(49, 0x542010010004180);
    print_bitboard(0x542010010004180);
    print_bitboard(foo);
    // while running {}
}
