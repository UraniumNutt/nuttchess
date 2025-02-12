mod board;
mod comm;
mod generate;
mod search;
pub mod tables;
use crate::board::*;
use crate::comm::*;
use core::panic;
use generate::generate;
use rand::seq::SliceRandom;
use search::perft;
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
}
