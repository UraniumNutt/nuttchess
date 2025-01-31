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
    println!("This is a test");
    let test = Tables::new();
    let rooks = test.rook_occupancy;
    let bishops = test.bishop_occupancy;
    let mask = rooks[0];
    let mut magic_test = [0; 64];
    // for index in 0..64 {
    //     let bit_count = test.relevent_rook_count[index];
    //     magic_test[index] = Tables::generate_magic(
    //         rooks[index],
    //         index,
    //         bit_count,
    //         &Tables::calculate_relevent_rook_occupancy,
    //     );
    //     println!("Found magic {:#016x} at index {}", magic_test[index], index);
    // }
    let magic = Tables::generate_magic(
        rooks[0],
        0,
        test.relevent_rook_count[0],
        &Tables::calculate_relevent_rook_occupancy,
    );
    println!("{}", magic);

    // let mut magic_test2 = [0; 64];
    // for index in 0..64 {
    //     let bit_count = test.relevent_bishop_count[index];
    //     magic_test[index] = Tables::generate_magic(
    //         bishops[index],
    //         index,
    //         bit_count,
    //         &Tables::calculate_relevent_bishops_occupancy,
    //     );
    //     println!("Found magic {:#016x} at index {}", magic_test[index], index);
    // }

    // while running {}
}
