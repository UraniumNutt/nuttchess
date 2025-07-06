use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    board::{BoardState, PieceType},
    tables::Tables,
};

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

    /// Debuging function to print the keys in the table
    fn print_keys(&self) {
        for piece_type in self.piece_keys {
            for key in piece_type {
                println!("{key:#018x}");
            }
        }
        for key in self.enpassant_keys {
            println!("{key:#018x}");
        }

        for key in self.castle_keys {
            println!("{key:#018x}");
        }

        println!("{:#018x}", self.side_key);
    }

    /// Debuging function to get some important keys
    fn print_special_keys(&self) {
        println!(
            "The en passant key of G6 is {:#018x}",
            self.enpassant_keys[Tables::G6 as usize]
        );
        println!(
            "The starting square key is {:#018x}",
            self.piece_keys[Self::WHITE_PAWN_INDEX][Tables::F2 as usize]
        );
        println!(
            "The ending square key is {:#018x}",
            self.piece_keys[Self::WHITE_PAWN_INDEX][Tables::F3 as usize]
        );
        println!("The side to mvoe key is {:#018x}", self.side_key);
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        board::{MoveRep, Promotion},
        generate::generate,
        tables::Tables,
    };

    use super::*;

    fn hash_test(starting_board: &str, final_board: &str, mv: MoveRep) {
        let mut starting_board = BoardState::state_from_string_fen(starting_board.to_string());
        let final_board = BoardState::state_from_string_fen(final_board.to_string());

        let zob_keys = &ZobKeys::new();
        let initial_hash = starting_board.hash;
        let final_hash = final_board.hash;

        starting_board.make(&mv, zob_keys);
        assert_eq!(starting_board.hash, final_hash);
        starting_board.unmake(&mv, zob_keys);
        assert_eq!(starting_board.hash, initial_hash);
    }

    fn perft_hash_test(starting_board: &str, depth: u64) {
        let mut starting_board = BoardState::state_from_string_fen(starting_board.to_string());
        let tables = Tables::new();
        let zob_keys = &ZobKeys::new();
        perft_hash_child(&mut starting_board, &tables, zob_keys, depth);
    }

    fn perft_hash_child(board: &mut BoardState, tables: &Tables, zob_keys: &ZobKeys, depth: u64) {
        if depth == 0 {
            return;
        }
        let moves = generate(board, tables);
        for mv in moves {
            let starting_hash = board.hash;
            board.make(&mv, zob_keys);
            perft_hash_child(board, tables, zob_keys, depth - 1);
            board.unmake(&mv, zob_keys);
            let final_hash = board.hash;
            assert_eq!(starting_hash, final_hash);
        }
    }

    #[ignore = "Takes a while"]
    #[test]
    fn perft_hash_inital_state_6() {
        perft_hash_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            6,
        );
    }

    #[test]
    fn perft_hash_kiwipete_4() {
        perft_hash_test(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            4,
        );
    }

    #[test]
    fn pawn_move_hash() {
        hash_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            MoveRep::new(
                1 << Tables::E2,
                1 << Tables::E4,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn knight_move_hash() {
        hash_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::G1,
                1 << Tables::F3,
                None,
                PieceType::Knight,
                None,
            ),
        );
    }

    #[test]
    fn bishop_move_hash() {
        hash_test(
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppp1ppp/8/1B2p3/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::F1,
                1 << Tables::B5,
                None,
                PieceType::Bishop,
                None,
            ),
        );
    }

    #[test]
    fn fools_mate_hash() {
        hash_test(
            "rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 1",
            "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 1",
            MoveRep::new(
                1 << Tables::D8,
                1 << Tables::H4,
                None,
                PieceType::Queen,
                None,
            ),
        );
    }

    #[test]
    fn promotion_hash() {
        hash_test(
            "rnb1kbnr/pppPpppp/8/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
            "rnbQkbnr/ppp1pppp/8/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::D7,
                1 << Tables::D8,
                Some(Promotion::Queen),
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn attack_hash() {
        hash_test(
            "rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::D4,
                1 << Tables::E5,
                None,
                PieceType::Pawn,
                Some(PieceType::Pawn),
            ),
        );
    }

    #[test]
    fn pawn_f2f4_hash() {
        hash_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1",
            MoveRep::new(
                1 << Tables::F2,
                1 << Tables::F4,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn pawn_f2f3_hash() {
        hash_test(
            "rnbqkbnr/pppppp2/8/6pp/8/6PP/PPPPPP2/RNBQKBNR w KQkq g6 0 1",
            "rnbqkbnr/pppppp2/8/6pp/8/5PPP/PPPPP3/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::F2,
                1 << Tables::F3,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn pawn_f2f3_no_enpassant_hash() {
        hash_test(
            "rnbqkbnr/pppppp2/6p1/7p/8/6PP/PPPPPP2/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppp2/6p1/7p/8/5PPP/PPPPP3/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::F2,
                1 << Tables::F3,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn perft_failure_hash() {
        hash_test(
            "rnbqkbnr/pppppp2/8/6pp/8/6PP/PPPPPP2/RNBQKBNR w KQkq g6 0 1",
            "rnbqkbnr/pppppp2/8/6pp/5P2/6PP/PPPPP3/RNBQKBNR b KQkq f3 0 1",
            MoveRep::new(
                1 << Tables::F2,
                1 << Tables::F4,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn en_passant_attacked_1() {
        hash_test(
            "rnbqkbnr/pppppp2/7p/6pP/8/8/PPPPPPP1/RNBQKBNR w KQkq g6 0 1",
            "rnbqkbnr/pppppp2/6Pp/8/8/8/PPPPPPP1/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::H5,
                1 << Tables::G6,
                None,
                PieceType::Pawn,
                Some(PieceType::Pawn),
            ),
        );
    }
    #[test]
    fn en_passant_not_attacked_1() {
        hash_test(
            "rnbqkbnr/pppppp2/7p/6pP/8/8/PPPPPPP1/RNBQKBNR w KQkq g6 0 1",
            "rnbqkbnr/pppppp2/7p/6pP/8/4P3/PPPP1PP1/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::E2,
                1 << Tables::E3,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }
    #[test]
    fn en_passant_attacked_2() {
        hash_test(
            "rnbqkbnr/ppp1pppp/8/2Pp4/8/8/PP1PPPPP/RNBQKBNR w KQkq d6 0 1",
            "rnbqkbnr/ppp1pppp/3P4/8/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::C5,
                1 << Tables::D6,
                None,
                PieceType::Pawn,
                Some(PieceType::Pawn),
            ),
        );
    }
    #[test]
    fn en_passant_not_attacked_2() {
        hash_test(
            "rnbqkbnr/ppp1pppp/8/2Pp4/8/8/PP1PPPPP/RNBQKBNR w KQkq d6 0 1",
            "rnbqkbnr/ppp1pppp/2P5/3p4/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1",
            MoveRep::new(
                1 << Tables::C5,
                1 << Tables::C6,
                None,
                PieceType::Pawn,
                None,
            ),
        );
    }

    #[test]
    fn castle_black_kingside() {
        hash_test(
            "rnbqk2r/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
            "rnbq1rk1/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQ - 0 1",
            MoveRep::new(
                1 << Tables::E8,
                1 << Tables::G8,
                Some(Promotion::Castle),
                PieceType::King,
                None,
            ),
        );
    }

    #[test]
    fn castle_black_queenside() {
        hash_test(
            "r3kbnr/ppp1pppp/3q4/3p4/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
            "2kr1bnr/ppp1pppp/3q4/3p4/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1",
            MoveRep::new(
                1 << Tables::E8,
                1 << Tables::C8,
                Some(Promotion::Castle),
                PieceType::King,
                None,
            ),
        );
    }

    #[test]
    fn castle_white_kingside() {
        hash_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 b kq - 0 1",
            MoveRep::new(
                1 << Tables::E1,
                1 << Tables::G1,
                Some(Promotion::Castle),
                PieceType::King,
                None,
            ),
        );
    }

    #[test]
    fn castle_white_queenside() {
        hash_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/2KR1BNR b kq - 0 1",
            MoveRep::new(
                1 << Tables::E1,
                1 << Tables::C1,
                Some(Promotion::Castle),
                PieceType::King,
                None,
            ),
        );
    }
}
