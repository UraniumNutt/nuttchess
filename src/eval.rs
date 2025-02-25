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

// Returns a score for the given board position
pub fn eval(board: &BoardState, tables: &Tables) -> isize {
    // Material evaluation
    material_value(board)
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

#[cfg(test)]
mod tests {
    use super::*;
}
