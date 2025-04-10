use std::path::StripPrefixError;

use crate::board::*;
use crate::generate::*;
use crate::tables::*;

// Constants
pub const WIN: isize = 10000;
pub const DRAW: isize = 0;

// Piece values
pub const KING: isize = 20000;
pub const QUEEN: isize = 900;
pub const ROOK: isize = 500;
pub const BISHOP: isize = 330;
pub const KNIGHT: isize = 320;
pub const PAWN: isize = 100;

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
    material_value(board)
        + (0.1 * (number_moves as f64 - last_number_moves as f64)) as isize
        + board.piece_square_score
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
pub fn delta_ps_score(board: &BoardState, mv: &MoveRep) -> isize {
    let mut score = 0;
    match mv.promotion {
        // Castling requries even more special handling
        Some(Promotion::Castle) => {
            match mv.ending_square {
                // White kingside
                e if e == (1 << Tables::G1) => {
                    // Remove king start score
                    score -= KING_TABLE[WHITE_MAP[mv.starting_square.trailing_zeros() as usize]];
                    // Add king end score
                    score += KING_TABLE[WHITE_MAP[mv.ending_square.trailing_zeros() as usize]];
                    // Remove rook start score
                    score -= ROOK_TABLE[WHITE_MAP[Tables::H1 as usize]];
                    // Add rook end score
                    score += ROOK_TABLE[WHITE_MAP[Tables::F1 as usize]];
                }
                // White queenside
                e if e == (1 << Tables::C1) => {
                    // Remove king start score
                    score -= KING_TABLE[WHITE_MAP[mv.starting_square.trailing_zeros() as usize]];
                    // Add king end score
                    score += KING_TABLE[WHITE_MAP[mv.ending_square.trailing_zeros() as usize]];
                    // Remove rook start score
                    score -= ROOK_TABLE[WHITE_MAP[Tables::A1 as usize]];
                    // Add rook end score
                    score += ROOK_TABLE[WHITE_MAP[Tables::D1 as usize]];
                }
                // Black kingside
                e if e == (1 << Tables::G8) => {
                    // Remove king start score
                    score -= KING_TABLE[BLACK_MAP[mv.starting_square.trailing_zeros() as usize]];
                    // Add king end score
                    score += KING_TABLE[BLACK_MAP[mv.ending_square.trailing_zeros() as usize]];
                    // Remove rook start score
                    score -= ROOK_TABLE[BLACK_MAP[Tables::H8 as usize]];
                    // Add rook end score
                    score += ROOK_TABLE[BLACK_MAP[Tables::F8 as usize]];
                }
                // Black queenside
                e if e == (1 << Tables::C8) => {
                    // Remove king start score
                    score -= KING_TABLE[BLACK_MAP[mv.starting_square.trailing_zeros() as usize]];
                    // Add king end score
                    score += KING_TABLE[BLACK_MAP[mv.ending_square.trailing_zeros() as usize]];
                    // Remove rook start score
                    score -= ROOK_TABLE[BLACK_MAP[Tables::A8 as usize]];
                    // Add rook end score
                    score += ROOK_TABLE[BLACK_MAP[Tables::D8 as usize]];
                }
                // No other case should occur!
                _ => panic!(),
            }
            score
        }
        // Everything else can be done in the same way
        _ => {
            // The moved piece should always be Some
            let (moved_piece, moved_color) = board.get_piece_and_color(mv.starting_square).unwrap();
            // Get the color corrected index into the piece square table
            let (move_start, move_end) = match moved_color {
                true => (
                    WHITE_MAP[mv.starting_square.trailing_zeros() as usize],
                    WHITE_MAP[mv.ending_square.trailing_zeros() as usize],
                ),
                false => (
                    BLACK_MAP[mv.starting_square.trailing_zeros() as usize],
                    BLACK_MAP[mv.ending_square.trailing_zeros() as usize],
                ),
            };
            // Adjust the score for the change of the moved piece
            score -= match moved_piece {
                PieceType::Pawn => PAWN_TABLE[move_start],
                PieceType::Knight => KNIGHT_TABLE[move_start],
                PieceType::Bishop => BISHOP_TABLE[move_start],
                PieceType::Rook => ROOK_TABLE[move_start],
                PieceType::Queen => QUEEN_TABLE[move_start],
                PieceType::King => KING_TABLE[move_start],
            };

            // Make sure to adjust if there is a promotion
            if mv.promotion.is_none() {
                score += match moved_piece {
                    PieceType::Pawn => PAWN_TABLE[move_end],
                    PieceType::Knight => KNIGHT_TABLE[move_end],
                    PieceType::Bishop => BISHOP_TABLE[move_end],
                    PieceType::Rook => ROOK_TABLE[move_end],
                    PieceType::Queen => QUEEN_TABLE[move_end],
                    PieceType::King => KING_TABLE[move_end],
                };
            }
            // If there is a promotion
            else {
                score += match mv.promotion.unwrap() {
                    Promotion::Queen => QUEEN_TABLE[move_end],
                    Promotion::Rook => ROOK_TABLE[move_end],
                    Promotion::Bishop => BISHOP_TABLE[move_end],
                    Promotion::Knight => KNIGHT_TABLE[move_end],
                    // This should never happen!
                    Promotion::Castle => panic!(),
                };
            }

            // Adjust the score for any attacked piece
            if let Some((attacked_piece, attacked_color)) =
                board.get_piece_and_color(mv.ending_square)
            {
                let attack_index = match attacked_color {
                    true => WHITE_MAP[mv.ending_square.trailing_zeros() as usize],
                    false => BLACK_MAP[mv.ending_square.trailing_zeros() as usize],
                };
                // Since the attacked piece is opposite to the side to move, removing a piece increased the current sides score
                score += match attacked_piece {
                    // Adjust for en passent moves
                    PieceType::Pawn => match board.en_passant_target {
                        // Normal case
                        0 => PAWN_TABLE[attack_index],
                        // En passant case
                        _ => {
                            let en_passant_index = match board.white_to_move {
                                true => {
                                    WHITE_MAP[board.en_passant_target.trailing_zeros() as usize]
                                }
                                false => {
                                    BLACK_MAP[board.en_passant_target.trailing_zeros() as usize]
                                }
                            };
                            PAWN_TABLE[en_passant_index]
                        }
                    },
                    PieceType::Knight => KNIGHT_TABLE[attack_index],
                    PieceType::Bishop => BISHOP_TABLE[attack_index],
                    PieceType::Rook => ROOK_TABLE[attack_index],
                    PieceType::Queen => QUEEN_TABLE[attack_index],
                    PieceType::King => KING_TABLE[attack_index],
                };
            }
            // Return the score
            score
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::search::perft;

    use super::*;

    #[test]
    fn dps_pawn_push_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::D2,
            1 << Tables::D4,
            None,
            PieceType::Pawn,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 40);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_pawn_push_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::D5,
            None,
            PieceType::Pawn,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 40);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_knight_move_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::G1,
            1 << Tables::F3,
            None,
            PieceType::Knight,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 50);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_knight_move_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::G8,
            1 << Tables::F6,
            None,
            PieceType::Knight,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 50);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_knight_move_3() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/N7/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::A4,
            1 << Tables::C5,
            None,
            PieceType::Knight,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 45);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_knight_move_4() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkb1r/ppppp1pp/5n2/8/N7/8/PPPPPPPP/R1BQKBNR b KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::F6,
            1 << Tables::G4,
            None,
            PieceType::Knight,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -5);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_bishop_move_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::F1,
            1 << Tables::C4,
            None,
            PieceType::Bishop,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 20);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_bishop_move_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppp1p/8/6p1/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::F8,
            1 << Tables::H6,
            None,
            PieceType::Bishop,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 0);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_rook_move_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppp1p/8/8/8/8/PPPPPPR1/RNBQKBN1 w Qkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::G2,
            1 << Tables::G7,
            None,
            PieceType::Rook,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 10);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_rook_move_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppp1p/8/8/8/8/PPPPPPR1/RNBQKBN1 b Qkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::A8,
            1 << Tables::A4,
            None,
            PieceType::Rook,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -5);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_rook_move_3() {
        let mut board = BoardState::state_from_string_fen(
            "r6r/1ppbbp1p/1nqppkn1/8/8/8/PPPPPP2/RNBQKBNR b - - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::A8,
            1 << Tables::D8,
            None,
            PieceType::Rook,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 5);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_rook_move_4() {
        let mut board = BoardState::state_from_string_fen(
            "r6r/1ppbbp1p/1nqppkn1/8/8/8/PPPPPP2/R1K4R w - - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::H1,
            1 << Tables::E1,
            None,
            PieceType::Rook,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 5);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_queen_move_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::D1,
            1 << Tables::D4,
            None,
            PieceType::Queen,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 10);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_queen_move_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbn1/pppppp1p/8/8/8/8/PQP1PPPP/RNB1KBNR w KQq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::B2,
            1 << Tables::H8,
            None,
            PieceType::Queen,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -20);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_king_move_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbn1/pppppp1p/8/8/8/8/PQP1KPPP/RNB2BNR w q - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E2,
            1 << Tables::E3,
            None,
            PieceType::King,
            None,
        );
        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -20);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_king_move_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqk1n1/ppppppbp/8/8/8/8/PQP1KPPP/RNB2BNR b q - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::F8,
            None,
            PieceType::King,
            None,
        );
        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 10);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_pawn_attack_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppp1ppp/8/8/8/5p2/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E2,
            1 << Tables::F3,
            None,
            PieceType::Pawn,
            Some(PieceType::Pawn),
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 30);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_pawn_attack_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/4P3/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E6,
            1 << Tables::D7,
            None,
            PieceType::Pawn,
            Some(PieceType::Pawn),
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 0);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_pawn_promo_1() {
        let mut board =
            BoardState::state_from_string_fen("8/3P2K1/8/8/8/8/6k1/8 w - - 0 1".to_string());

        let mv = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::D8,
            Some(Promotion::Queen),
            PieceType::Pawn,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -55);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_pawn_promo_2() {
        let mut board =
            BoardState::state_from_string_fen("4r3/3P2K1/8/8/8/8/6k1/8 w - - 0 1".to_string());

        let mv = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::E8,
            Some(Promotion::Queen),
            PieceType::Pawn,
            Some(PieceType::Rook),
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -50);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_pawn_promo_3() {
        let mut board =
            BoardState::state_from_string_fen("8/6K1/8/8/8/8/3p2k1/4R3 b - - 0 1".to_string());

        let mv = MoveRep::new(
            1 << Tables::D2,
            1 << Tables::E1,
            Some(Promotion::Queen),
            PieceType::Pawn,
            Some(PieceType::Rook),
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, -55);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_castle_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/5NPB/PPPPPP1P/RNBQK2R w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::G1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 30);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_castle_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/BPN1PQ2/P1PP1PPP/R3KBNR w KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::C1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 15);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_castle_3() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqk2r/pppp1ppp/3bpn2/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::G8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 30);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_castle_4() {
        let mut board = BoardState::state_from_string_fen(
            "r3kbnr/ppp1pppp/2nqb3/3p4/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let mv = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::C8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let delta_score = delta_ps_score(&board, &mv);
        assert_eq!(delta_score, 15);
        let init_score = piece_square_score(&board);
        board.make(&mv);
        let final_score = piece_square_score(&board);
        // The final score is a diffrent sign because the side to move is diffrent
        assert_eq!(delta_score, -(init_score + final_score));
        board.unmake(&mv);
        let revert_score = piece_square_score(&board);
        assert_eq!(init_score, revert_score);
    }

    #[test]
    fn dps_balanced() {
        let mut board = BoardState::starting_state();
        let init_score = board.piece_square_score;
        let _ = perft(&mut board, 5);
        assert_eq!(board.piece_square_score, init_score);
    }
}
