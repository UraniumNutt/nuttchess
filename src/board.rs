struct BoardState {
    white_pawns: u64,
    white_knights: u64,
    white_rooks: u64,
    white_bishops: u64,
    white_queens: u64,
    white_king: u64,
    white_queenside_castle_rights: bool,
    white_kingside_castle_rights: bool,

    black_pawns: u64,
    black_knights: u64,
    black_rooks: u64,
    black_bishops: u64,
    black_queens: u64,
    black_king: u64,
    black_queenside_castle_rights: bool,
    black_kingside_castle_rights: bool,

    white_to_move: bool,
    en_passant_target: u64,
    reversable_move_counter: u8,
}

impl BoardState {
    pub fn starting_state() -> BoardState {
        BoardState {
            white_pawns: 0xff00,
            white_knights: 0x42,
            white_rooks: 0x81,
            white_bishops: 0x24,
            white_queens: 0x8,
            white_king: 0x10,
            white_queenside_castle_rights: true,
            white_kingside_castle_rights: true,

            black_pawns: 0xff000000000000,
            black_knights: 0x4200000000000000,
            black_rooks: 0x8100000000000000,
            black_bishops: 0x2400000000000000,
            black_queens: 0x800000000000000,
            black_king: 0x1000000000000000,
            black_queenside_castle_rights: true,
            black_kingside_castle_rights: true,

            white_to_move: true,
            en_passant_target: 0x0,
            reversable_move_counter: 0,
        }
    }
}
