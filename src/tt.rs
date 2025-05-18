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
    use crate::board::MoveRep;

    use super::*;

    // #[test]
    // fn print_keys() {
    //     let keys = ZobKeys::new();

    //     for square in 0..64 {
    //         for piece_index in 0..12 {
    //             println!("{:#018x}", keys.piece_keys[piece_index][square]);
    //         }
    //     }

    //     for square in 0..64 {
    //         println!("{:#018x}", keys.enpassant_keys[square]);
    //     }

    //     for castle_state in 0..4 {
    //         println!("{:#018x}", keys.castle_keys[castle_state]);
    //     }

    //     println!("{:#018x}", keys.side_key);

    //     panic!();
    // }
    //
    // #[test]
    // fn gen_key() {
    //     let keys = ZobKeys::new();
    //     let board = BoardState::starting_state();
    //     let generated_hash = keys.generate_hash(&board);
    //     println!("{:#018x}", generated_hash);
    //     let board2 = BoardState::state_from_string_fen(
    //         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
    //     );
    //     let hash2 = keys.generate_hash(&board2);
    //     println!("{:#018x}", hash2);
    //     panic!();
    // }
}
