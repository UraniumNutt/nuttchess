mod board;
mod comm;
mod generate;
mod search;
pub mod tables;
use crate::board::*;
use crate::comm::*;
use search::perft;
use search::perft_search;
use std::env;
use std::path::Path;
use tables::Tables;

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth = args[1].to_owned();
    let fen = args[2].to_owned();
    let moves_list = match args.len() {
        4 => args[3].to_owned(),
        _ => "".to_owned(),
    };
    let moves = moves_list.split(" ");
    let log_file = Path::new("log.txt");
    let mut comm = Comm::create(log_file).unwrap();
    let mut board = BoardState::state_from_fen(fen.split(" ")).unwrap();
    if moves_list.len() != 0 {
        for mv in moves {
            board.apply_string_move(mv.to_string());
        }
    }
    perft(&mut board, depth.parse::<u64>().unwrap() as usize);

    // Generate magics
    // let mut rook_magics = [0; 64];
    // let mut bishop_magics = [0; 64];
    // let mut rook_mask = [0; 64];
    // let mut bishop_mask = [0; 64];
    // Tables::generate_rook_occupancy_mask(&mut rook_mask);
    // Tables::generate_bishop_occupancy_mask(&mut bishop_mask);
    // for square in 0..64 {
    //     rook_magics[square] = Tables::generate_magic(
    //         rook_mask[square],
    //         square,
    //         rook_mask[square].count_ones() as u64,
    //         &Tables::calculate_relevent_rook_occupancy,
    //     );
    //     bishop_magics[square] = Tables::generate_magic(
    //         bishop_mask[square],
    //         square,
    //         bishop_mask[square].count_ones() as u64,
    //         &Tables::calculate_relevent_bishop_occupancy,
    //     );
    // }

    // println!("Rook magics: ");
    // for square in 0..64 {
    //     println!("{:#018x}", rook_magics[square]);
    // }

    // println!("Bishop magics: ");
    // for square in 0..64 {
    //     println!("{:#018x}", bishop_magics[square]);
    // }
}
