use std::{
    borrow::Borrow,
    fmt::{Debug, Formatter, Write},
};

use crate::uci::{GuiToEngine, PositionType};

#[derive(Clone, Copy, Debug)]
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

pub fn match_piece(piece: &Piece) -> char {
    match piece {
        Piece::WhitePawn => '♟',
        Piece::WhiteKnight => '♞',
        Piece::WhiteBishop => '♝',
        Piece::WhiteRook => '♜',
        Piece::WhiteQueen => '♛',
        Piece::WhiteKing => '♚',

        Piece::BlackPawn => '♙',
        Piece::BlackKnight => '♘',
        Piece::BlackBishop => '♗',
        Piece::BlackRook => '♖',
        Piece::BlackQueen => '♕',
        Piece::BlackKing => '♔',

        Piece::None => ' ',
    }
}

pub struct BoardState {
    board: [Piece; 64],
    white_castle_rights_queen_side: bool,
    white_castle_rights_king_side: bool,
    black_castle_rights_queen_side: bool,
    black_castle_rights_king_side: bool,
    en_passant_target: Option<usize>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            board: START_BOARD,
            white_castle_rights_queen_side: true,
            white_castle_rights_king_side: true,
            black_castle_rights_queen_side: true,
            black_castle_rights_king_side: true,
            en_passant_target: None,
        }
    }
}

pub fn board_state_from_pos(position: &PositionType) -> Option<BoardState> {
    match position {
        PositionType::FenString(string) => board_state_from_fen(string),
        PositionType::StartPos(string) => board_state_from_startpos(string),
    }
}

fn board_state_from_fen(fen: &Vec<String>) -> Option<BoardState> {
    println!("{:?}", fen);
    Some(BoardState::new())
}

fn board_state_from_startpos(startpos: &Vec<String>) -> Option<BoardState> {
    println!("{:?}", startpos);
    Some(BoardState::new())
}

impl Debug for BoardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.board.iter().enumerate() {
            if i % 8 == 0 {
                f.write_char('\n')?;
            }
            f.write_fmt(format_args!("{} ", match_piece(val)))?;
        }
        Ok(())
    }
}

// pub fn state_from_string(position: &PositionType) -> Option(BoardState) {
//     Some(BoardState::new())
// }

const START_BOARD: [Piece; 64] = [
    Piece::BlackRook,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackQueen,
    Piece::BlackKing,
    Piece::BlackBishop,
    Piece::BlackKnight,
    Piece::BlackRook,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::BlackPawn,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::None,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhitePawn,
    Piece::WhiteRook,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteQueen,
    Piece::WhiteKing,
    Piece::WhiteBishop,
    Piece::WhiteKing,
    Piece::WhiteRook,
];
