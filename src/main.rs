mod board;
mod comm;
mod eval;
mod generate;
mod search;
pub mod tables;
use crate::board::*;
use crate::comm::*;
use crate::tables::*;
use search::negamax;
use search::perft;
use std::env;
use std::path::Path;

fn main() {
    let mut running = true;
    let log_file = Path::new("log.txt");
    let mut comm = Comm::create(log_file).unwrap();

    let mut board = BoardState::starting_state();
    let tables = Tables::new();
    while running {
        let line = comm.engine_in();
        let mut tokens = line.split(" ");
        match tokens.next().unwrap() {
            "uci" => {
                comm.engine_out("id name nuttchess".to_string());
                comm.engine_out("id author UraniumNutt / Ethan Thummel".to_string());
                comm.engine_out("uciok".to_string());
            }
            "isready" => {
                comm.engine_out("readyok".to_string());
            }
            "ucinewgame" => {}
            "position" => match tokens.next().unwrap() {
                "startpos" => {
                    board = BoardState::starting_state();
                    if tokens.next().unwrap() == "moves" {
                        while let Some(mv) = tokens.next() {
                            board.apply_string_move(mv.to_string());
                        }
                    } else {
                        comm.engine_out(format!("Expected moves token"));
                    }
                }
                "fen" => {
                    let parsed_board = BoardState::state_from_fen(&mut tokens);
                    match parsed_board {
                        Ok(b) => {
                            board = b;
                        }
                        Err(b) => {
                            comm.engine_out(format!("Error parsing fen string: {}", b));
                        }
                    }
                    if tokens.next().unwrap() == "moves" {
                        while let Some(mv) = tokens.next() {
                            board.apply_string_move(mv.to_string());
                        }
                    } else {
                        comm.engine_out(format!("Expected moves token"));
                    }
                }
                e => comm.engine_out(format!("Unexpected value {}", e)),
            },
            "go" => match tokens.next().unwrap() {
                "perft" => {
                    let depth = tokens.next();
                    match depth {
                        Some(d) => {
                            if let Ok(parsed_d) = d.parse::<u64>() {
                                perft(&mut board, parsed_d as usize);
                            } else {
                                comm.engine_out(format!(
                                    "Error parsing value of depth token {}",
                                    d
                                ));
                            }
                        }
                        None => {
                            comm.engine_out(format!("Expected depth after token perft"));
                        }
                    }
                }
                // TODO implement more / the proper searches
                "wtime" => {
                    // Just discard these for now
                    let _ = tokens.next();
                    let _ = tokens.next();
                    let _ = tokens.next();
                    let best_move = negamax(&mut board, &tables, 6).unwrap();
                    comm.engine_out(format!("bestmove {}", best_move.to_string().unwrap()));
                }
                _ => {}
            },
            "quit" => {
                running = false;
            }
            _ => {}
        }
    }

    // Use this to hook up the perft tester
    // let args: Vec<String> = env::args().collect();
    // let depth = args[1].to_owned();
    // let fen = args[2].to_owned();
    // let moves_list = match args.len() {
    //     4 => args[3].to_owned(),
    //     _ => "".to_owned(),
    // };
    // let moves = moves_list.split(" ");
    // let log_file = Path::new("log.txt");
    // let mut comm = Comm::create(log_file).unwrap();
    // let mut board = BoardState::state_from_fen(fen.split(" ")).unwrap();
    // if moves_list.len() != 0 {
    //     for mv in moves {
    //         board.apply_string_move(mv.to_string());
    //     }
    // }
    // perft(&mut board, depth.parse::<u64>().unwrap() as usize);
}
