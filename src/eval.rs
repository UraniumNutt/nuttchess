use crate::board::*;
use crate::generate::*;
use crate::tables::*;

// Constants
pub const WIN: isize = 10000;
pub const DRAW: isize = 0;

// Piece values
const KING: isize = 200;
const QUEEN: isize = 9;
const ROOK: isize = 5;
const BISHOP: isize = 3;
const KNIGHT: isize = 3;
const PAWN: isize = 1;

// Piece Square Tables - Taken from https://www.chessprogramming.org/Simplified_Evaluation_Function
#[rustfmt::skip]
const PAWN_TABLE: [isize; 64] = [ 
     0,  0,  0,  0,  0,  0,  0,  0, 
    50, 50, 50, 50, 50, 50, 50, 50,  
    10, 10, 20, 30, 30, 20, 10, 10,  
     5,  5, 10, 25, 25, 10,  5,  5,  
     0,  0,  0, 20, 20,  0,  0,  0,  
     5, -5,-10,  0,  0,-10, -5,  5,  
     5, 10, 10,-20,-20, 10, 10,  5,  
     0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
const KNIGHT_TABLE: [isize; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50, 
];

#[rustfmt::skip]
const BISHOP_TABLE: [isize; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20, 
];

#[rustfmt::skip]
const ROOK_TABLE: [isize; 64] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      5, 10, 10, 10, 10, 10, 10,  5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
      0,  0,  0,  5,  5,  0,  0,  0 
];

#[rustfmt::skip]
const QUEEN_TABLE: [isize; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20 
];

#[rustfmt::skip]
const KING_TABLE: [isize; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20 
];

// Returns a score for the given board position
pub fn eval(
    board: &BoardState,
    tables: &Tables,
    number_moves: usize,
    last_number_moves: usize,
) -> isize {
    // Material evaluation
    material_value(board) + (0.1 * (number_moves as f64 - last_number_moves as f64)) as isize
}

// Get the value of the material relative to the side to move
pub fn material_value(board: &BoardState) -> isize {
    // White relative value
    let king_delta =
        board.white_king.count_ones() as isize - board.black_king.count_ones() as isize;
    let queen_delta =
        board.white_queens.count_ones() as isize - board.black_queens.count_ones() as isize;
    let rook_delta =
        board.white_rooks.count_ones() as isize - board.black_rooks.count_ones() as isize;
    let bishop_delta =
        board.white_bishops.count_ones() as isize - board.black_bishops.count_ones() as isize;
    let knight_delta = board.white_knights.count_ones() as isize as isize
        - board.black_knights.count_ones() as isize;
    let pawn_delta =
        board.white_pawns.count_ones() as isize - board.black_pawns.count_ones() as isize;

    // Get the total value
    let white_relative_value = KING * king_delta
        + QUEEN * queen_delta
        + ROOK * rook_delta
        + BISHOP * bishop_delta
        + KNIGHT * knight_delta
        + PAWN * pawn_delta;
    if board.white_to_move {
        return white_relative_value;
    } else {
        return -white_relative_value;
    }
}

/// Convert piece index to piece square table index
fn convert_index(index: usize, white_to_move: bool) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
}
