use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::board::{BoardState, PieceType};

pub struct ZobKeys {
    // Piece order:
    // WHITE Pawn Knight Bishop Rook Queen King
    // BLACK Pawn Knight Bishop Rook Queen King
    pub piece_keys: [[u64; 64]; 12],
    pub enpassant_keys: [u64; 64],
    // WHITE kingside, queenside
    // BLACK kingside, queenside
    pub castle_keys: [u64; 4],
    pub side_key: u64,
}

impl ZobKeys {
    pub const WHITE_PAWN_INDEX: usize = 0;
    pub const WHITE_KNIGHT_INDEX: usize = 1;
    pub const WHITE_BISHOP_INDEX: usize = 2;
    pub const WHITE_ROOK_INDEX: usize = 3;
    pub const WHITE_QUEEN_INDEX: usize = 4;
    pub const WHITE_KING_INDEX: usize = 5;
    pub const BLACK_PAWN_INDEX: usize = 6;
    pub const BLACK_KNIGHT_INDEX: usize = 7;
    pub const BLACK_BISHOP_INDEX: usize = 8;
    pub const BLACK_ROOK_INDEX: usize = 9;
    pub const BLACK_QUEEN_INDEX: usize = 10;
    pub const BLACK_KING_INDEX: usize = 11;

    pub const WHITE_KINGSIDE_INDEX: usize = 0;
    pub const WHITE_QUEENSIDE_INDEX: usize = 1;
    pub const BLACK_KINGSIDE_INDEX: usize = 2;
    pub const BLACK_QUEENSIDE_INDEX: usize = 3;

    pub fn new() -> ZobKeys {
        let mut keys = ZobKeys {
            piece_keys: [[0; 64]; 12],
            enpassant_keys: [0; 64],
            castle_keys: [0; 4],
            side_key: 0,
        };

        // Derive the keys from xor shift so they are always the same
        let mut rng = XorShiftRng::seed_from_u64(0);

        // Init the piece_keys
        for square in 0..64 {
            for piece_index in 0..12 {
                keys.piece_keys[piece_index][square] = rng.next_u64();
            }
        }

        // Init the enpassant_keys
        for square in 0..64 {
            keys.enpassant_keys[square] = rng.next_u64();
        }

        // Init the castling state
        for castle_state in 0..4 {
            keys.castle_keys[castle_state] = rng.next_u64();
        }

        // Init the side key
        keys.side_key = rng.next_u64();

        keys
    }

    /// Match the piece and side to move to the index
    pub fn match_to_index(piece: PieceType, white_to_move: bool) -> usize {
        match (piece, white_to_move) {
            (PieceType::Pawn, true) => ZobKeys::WHITE_PAWN_INDEX,
            (PieceType::Knight, true) => ZobKeys::WHITE_KNIGHT_INDEX,
            (PieceType::Bishop, true) => ZobKeys::WHITE_BISHOP_INDEX,
            (PieceType::Rook, true) => ZobKeys::WHITE_ROOK_INDEX,
            (PieceType::Queen, true) => ZobKeys::WHITE_QUEEN_INDEX,
            (PieceType::King, true) => ZobKeys::WHITE_KING_INDEX,
            (PieceType::Pawn, false) => ZobKeys::BLACK_PAWN_INDEX,
            (PieceType::Knight, false) => ZobKeys::BLACK_KNIGHT_INDEX,
            (PieceType::Bishop, false) => ZobKeys::BLACK_BISHOP_INDEX,
            (PieceType::Rook, false) => ZobKeys::BLACK_ROOK_INDEX,
            (PieceType::Queen, false) => ZobKeys::BLACK_QUEEN_INDEX,
            (PieceType::King, false) => ZobKeys::BLACK_KING_INDEX,
        }
    }

    /// Initially generate a hash from a board state
    pub fn generate_hash(&self, board: &BoardState) -> u64 {
        let mut hash = 0;
        // Hash if the side to move is white
        if board.white_to_move {
            hash ^= self.side_key;
        }
        // Castle rights
        if board.white_kingside_castle_rights {
            hash ^= self.castle_keys[0];
        }
        if board.white_queenside_castle_rights {
            hash ^= self.castle_keys[1];
        }
        if board.black_kingside_castle_rights {
            hash ^= self.castle_keys[2];
        }
        if board.black_queenside_castle_rights {
            hash ^= self.castle_keys[3];
        }
        // Piece keys
        for index in 0..64 {
            if let Some(piece) = board.get_piece_and_color(1 << index) {
                let piece_type_offset = match piece {
                    (PieceType::Pawn, true) => 0,
                    (PieceType::Knight, true) => 1,
                    (PieceType::Bishop, true) => 2,
                    (PieceType::Rook, true) => 3,
                    (PieceType::Queen, true) => 4,
                    (PieceType::King, true) => 5,
                    (PieceType::Pawn, false) => 6,
                    (PieceType::Knight, false) => 7,
                    (PieceType::Bishop, false) => 8,
                    (PieceType::Rook, false) => 9,
                    (PieceType::Queen, false) => 10,
                    (PieceType::King, false) => 11,
                };
                hash ^= self.piece_keys[piece_type_offset][index];
            }
        }
        // En passant square
        if board.en_passant_target & 0xff0000ff0000 != 0 {
            hash ^= self.enpassant_keys[board.en_passant_target.trailing_zeros() as usize];
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::MoveRep, tables::Tables};

    use super::*;

    #[test]
    fn pawn_move_hash() {
        let mut starting_board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let mut final_board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string(),
        );
        let zob_keys = &ZobKeys::new();
        let initial_hash = starting_board.hash;
        let final_hash = final_board.hash;

        let mv = MoveRep::new(
            1 << Tables::E2,
            1 << Tables::E4,
            None,
            PieceType::Pawn,
            None,
        );

        starting_board.make(&mv, zob_keys);
        assert_eq!(starting_board.hash, final_hash);
        starting_board.unmake(&mv, zob_keys);
        assert_eq!(starting_board.hash, initial_hash);
    }

    #[test]
    fn knight_move_hash() {
        let mut starting_board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let mut final_board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 1".to_string(),
        );
        let zob_keys = &ZobKeys::new();
        let initial_hash = starting_board.hash;
        let final_hash = final_board.hash;

        let mv = MoveRep::new(
            1 << Tables::G1,
            1 << Tables::F3,
            None,
            PieceType::Knight,
            None,
        );

        starting_board.make(&mv, zob_keys);
        assert_eq!(starting_board.hash, final_hash);
        starting_board.unmake(&mv, zob_keys);
        assert_eq!(starting_board.hash, initial_hash);
    }
}
