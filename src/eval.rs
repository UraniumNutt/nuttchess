use std::path::StripPrefixError;

use crate::board::*;
use crate::generate::*;
use crate::tables::*;

// Constants
pub const WIN: isize = 10000;
pub const DRAW: isize = 0;

// Piece values
pub const KING: isize = 200;
pub const QUEEN: isize = 9;
pub const ROOK: isize = 5;
pub const BISHOP: isize = 3;
pub const KNIGHT: isize = 3;
pub const PAWN: isize = 1;

// Piece Square Tables - Taken from https://www.chessprogramming.org/Simplified_Evaluation_Function
#[rustfmt::skip]
pub const PAWN_TABLE: [isize; 64] = [ 
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
pub const KNIGHT_TABLE: [isize; 64] = [
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
pub const BISHOP_TABLE: [isize; 64] = [
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
pub const ROOK_TABLE: [isize; 64] = [
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
pub const QUEEN_TABLE: [isize; 64] = [
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
pub const KING_TABLE: [isize; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20 
];

#[rustfmt::skip]
pub const WHITE_MAP: [usize; 64] = [
    63, 62, 61, 60, 59, 58, 57, 56,  
    55, 54, 53, 52, 51, 50, 49, 48,  
    47, 46, 45, 44, 43, 42, 41, 40,  
    39, 38, 37, 36, 35, 34, 33, 32,  
    31, 30, 29, 28, 27, 26, 25, 24,  
    23, 22, 21, 20, 19, 18, 17, 16,  
    15, 14, 13, 12, 11, 10,  9,  8,  
     7,  6,  5,  4,  3,  2,  1,  0,  
];

#[rustfmt::skip]
pub const BLACK_MAP: [usize; 64] = [
    7,  6,  5,  5,  3,   2,  1,  0,  
    15, 14, 13, 12, 11, 10,  9,  8,  
    23, 22, 21, 20, 19, 18, 17, 16,  
    31, 30, 29, 28, 27, 26, 25, 24,  
    39, 38, 37, 36, 35, 34, 33, 32,  
    47, 46, 45, 44, 43, 42, 41, 40,  
    55, 54, 53, 52, 51, 50, 49, 48,  
    63, 62, 61, 60, 59, 58, 57, 56,  
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
    // + piece_square_score(board)
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

/// Get the boards piece square value
pub fn piece_square_score(board: &BoardState) -> isize {
    // Start to score as if white is to move
    let mut white_score = 0;
    let mut black_score = 0;

    let mut white_pawns = board.white_pawns;
    while white_pawns != 0 {
        white_score += PAWN_TABLE[WHITE_MAP[pop_lsb(&mut white_pawns)]];
    }
    let mut white_knights = board.white_knights;
    while white_knights != 0 {
        white_score += KNIGHT_TABLE[WHITE_MAP[pop_lsb(&mut white_knights)]];
    }
    let mut white_rooks = board.white_rooks;
    while white_rooks != 0 {
        white_score += ROOK_TABLE[WHITE_MAP[pop_lsb(&mut white_rooks)]];
    }
    let mut white_bishops = board.white_bishops;
    while white_bishops != 0 {
        white_score += BISHOP_TABLE[WHITE_MAP[pop_lsb(&mut white_bishops)]];
    }
    let mut white_queens = board.white_queens;
    while white_queens != 0 {
        white_score += QUEEN_TABLE[WHITE_MAP[pop_lsb(&mut white_queens)]];
    }
    let mut white_king = board.white_king;
    while white_king != 0 {
        white_score += KING_TABLE[WHITE_MAP[pop_lsb(&mut white_king)]];
    }

    let mut black_pawns = board.black_pawns;
    while black_pawns != 0 {
        black_score += PAWN_TABLE[BLACK_MAP[pop_lsb(&mut black_pawns)]];
    }
    let mut black_knights = board.black_knights;
    while black_knights != 0 {
        black_score += KNIGHT_TABLE[BLACK_MAP[pop_lsb(&mut black_knights)]];
    }
    let mut black_rooks = board.black_rooks;
    while black_rooks != 0 {
        black_score += ROOK_TABLE[BLACK_MAP[pop_lsb(&mut black_rooks)]];
    }
    let mut black_bishops = board.black_bishops;
    while black_bishops != 0 {
        black_score += BISHOP_TABLE[BLACK_MAP[pop_lsb(&mut black_bishops)]];
    }
    let mut black_queens = board.black_queens;
    while black_queens != 0 {
        black_score += QUEEN_TABLE[BLACK_MAP[pop_lsb(&mut black_queens)]];
    }
    let mut black_king = board.black_king;
    while black_king != 0 {
        black_score += KING_TABLE[BLACK_MAP[pop_lsb(&mut black_king)]];
    }

    match board.white_to_move {
        true => (white_score - black_score) as isize,
        false => (black_score - white_score) as isize,
    }
}

/// Get the value of a piece at the mask
pub fn get_piece_value(board: &BoardState, mask: u64) -> isize {
    let piece_color = board.get_piece_and_color(mask);
    if let Some(piece_color) = piece_color {
        let white_index = WHITE_MAP[mask.trailing_zeros() as usize];
        let black_index = BLACK_MAP[mask.trailing_zeros() as usize];
        match piece_color {
            (PieceType::Pawn, true) => PAWN_TABLE[white_index] as isize,
            (PieceType::Knight, true) => KNIGHT_TABLE[white_index] as isize,
            (PieceType::Bishop, true) => BISHOP_TABLE[white_index] as isize,
            (PieceType::Rook, true) => ROOK_TABLE[white_index] as isize,
            (PieceType::Queen, true) => QUEEN_TABLE[white_index] as isize,
            (PieceType::King, true) => KING_TABLE[white_index] as isize,
            (PieceType::Pawn, false) => PAWN_TABLE[black_index] as isize,
            (PieceType::Knight, false) => KNIGHT_TABLE[black_index] as isize,
            (PieceType::Bishop, false) => BISHOP_TABLE[black_index] as isize,
            (PieceType::Rook, false) => ROOK_TABLE[black_index] as isize,
            (PieceType::Queen, false) => QUEEN_TABLE[black_index] as isize,
            (PieceType::King, false) => KING_TABLE[black_index] as isize,
        }
    } else {
        return 0;
    }
}

/// Get the delta piece square score from a MoveRep
pub fn delta_ps_score(mv: &MoveRep) -> isize {
    let mut score = 0;
    match mv.promotion {
        None => {
            score += 10;
            score
        }
        Some(Promotion::Queen) => score,
        Some(Promotion::Rook) => score,
        Some(Promotion::Bishop) => score,
        Some(Promotion::Knight) => score,
        Some(Promotion::Castle) => score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
