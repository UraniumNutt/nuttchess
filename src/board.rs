
#[derive(Clone)]
#[derive(Copy)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,

    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,

    None,
}

pub struct BoardState {
    board: [Piece; 64],
    white_castle_rights: bool,
    black_castle_rights: bool,
    en_passant_target: Option<usize>
}

impl BoardState {
    pub fn new() -> Self{

        Self {
            board: [Piece::None; 64],
            white_castle_rights: true,
            black_castle_rights: true,
            en_passant_target: None,
        }

    }
}