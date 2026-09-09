use std::fmt;
use crate::board::Board;
use crate::piece;

pub const CAPTURE: u8 =          0b00000001;
pub const PROMOTION: u8 =        0b00000010;
pub const EN_PASSANT: u8 =       0b00000100;
pub const CASTLING: u8 =         0b00001000;
pub const DOUBLE_PAWN_PUSH: u8 = 0b00010000;

#[derive(Clone, Copy)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub flags: u8
}

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to, flags: 0 }
    }

    pub fn new_with_flags(from: usize, to: usize, flags: u8) -> Self {
        Self { from, to, flags }
    }

    pub fn is_capture(&self) -> bool {
        self.flags & CAPTURE != 0
    }

    pub fn is_promotion(&self) -> bool {
        self.flags & PROMOTION != 0
    }

    pub fn is_en_passant(&self) -> bool {
        self.flags & EN_PASSANT != 0
    }

    pub fn is_castling(&self) -> bool {
        self.flags & CASTLING != 0
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.flags & DOUBLE_PAWN_PUSH != 0
    }

    pub fn get_promotion_piece(&self) -> u8 {
        if self.flags >> 5 == 0 {
            return piece::QUEEN;
        } else if (self.flags >> 5) & 1 != 0 {
            return piece::ROOK;
        } else if (self.flags >> 5) & 2 != 0 {
            return piece::BISHOP;
        } else if (self.flags >> 5) & 4 != 0 {
            return piece::KNIGHT;
        }
        return piece::QUEEN;
    }

    pub fn set_promotion_piece(mut self, piece: u8) -> Self {
        let mut flag = 0;
        if piece == piece::QUEEN {
            flag = 0;
        } else if piece == piece::ROOK {
            flag = 1;
        } else if piece == piece::BISHOP {
            flag = 2;
        } else if piece == piece::KNIGHT {
            flag = 4;
        }
        self.flags |= flag << 5;
        self
    }

    pub fn to_string(&self) -> String {
        let from_file = (self.from % 8) as u8 + b'a';
        let from_rank = 8 - (self.from / 8) as u8 + b'0';
        let to_file = (self.to % 8) as u8 + b'a';
        let to_rank = 8 - (self.to / 8) as u8 + b'0';
        format!("{}{}{}{}", char::from(from_file), char::from(from_rank), char::from(to_file), char::from(to_rank))
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

pub fn get_king_moves(board: &Board, pos: usize, moves: &mut Vec<Move>) {
    let king = board.get_square(pos);

    let offsets: [isize; 8] = [
        -9, -8, -7, -1, 1, 7, 8, 9
    ];
    for offset in offsets {
        let new_pos = pos as isize + offset;
        if new_pos < 0 || new_pos >= 64 {
            continue;
        }
        if (new_pos as usize % 8).abs_diff(pos % 8) > 1 || (new_pos as usize / 8).abs_diff(pos / 8) > 1 {
            continue;
        }
        if piece::get_color(board.get_square(new_pos as usize)) == piece::get_color(king) {
            continue;
        }
        if piece::get_color(board.get_square(new_pos as usize)) != piece::get_color(king) && !piece::is_empty(board.get_square(new_pos as usize)) {
            moves.push(Move::new_with_flags(pos, new_pos as usize, CAPTURE));
            continue;
        }
        moves.push(Move::new(pos, new_pos as usize));
    }

    if pos == 4 {
        if board.get_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_KING_SIDE) {
            moves.push(Move::new_with_flags(pos, pos + 2, CASTLING));
        }
        if board.get_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_QUEEN_SIDE) {
            moves.push(Move::new_with_flags(pos, pos - 2, CASTLING));
        }
    } else if pos == 60 {
        if board.get_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_KING_SIDE) {
            moves.push(Move::new_with_flags(pos, pos + 2, CASTLING));
        }
        if board.get_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_QUEEN_SIDE) {
            moves.push(Move::new_with_flags(pos, pos - 2, CASTLING));
        }
    }
}

pub fn get_knight_moves(board: &Board, pos: usize, moves: &mut Vec<Move>) {
    let knight = board.get_square(pos);

    let offsets: [isize; 8] = [
        -17, -15, -10, -6, 6, 10, 15, 17
    ];
    for offset in offsets {
        let new_pos = pos as isize + offset;
        if new_pos < 0 || new_pos >= 64 {
            continue;
        }
        if (new_pos as usize % 8).abs_diff(pos % 8) > 2 || (new_pos as usize / 8).abs_diff(pos / 8) > 2 {
            continue;
        }
        if piece::get_color(board.get_square(new_pos as usize)) == piece::get_color(knight) {
            continue;
        }
        if piece::get_color(board.get_square(new_pos as usize)) != piece::get_color(knight) && !piece::is_empty(board.get_square(new_pos as usize)) {
            moves.push(Move::new_with_flags(pos, new_pos as usize, CAPTURE));
            continue;
        }
        moves.push(Move::new(pos, new_pos as usize));
    }
}

pub fn get_pawn_moves(board: &Board, pos: usize, moves: &mut Vec<Move>) {
    let pawn = board.get_square(pos);

    let offset = if piece::get_color(pawn) == piece::WHITE { -8 } else { 8 };
    let new_pos = pos as isize + offset;
    if new_pos >= 0 && new_pos < 64 && piece::is_empty(board.get_square(new_pos as usize)) {
        if new_pos / 8 == 7 || new_pos / 8 == 0 {
            moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION));
            moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION).set_promotion_piece(piece::ROOK));
            moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION).set_promotion_piece(piece::BISHOP));
            moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION).set_promotion_piece(piece::KNIGHT));

            // moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION).set_promotion_piece(board.get_promotion_piece()));
        } else {
            moves.push(Move::new(pos, new_pos as usize));
        }
    }
    
    if (pos / 8 == 1 && piece::get_color(pawn) == piece::BLACK) || (pos / 8 == 6 && piece::get_color(pawn) == piece::WHITE) {
        let new_pos = pos as isize + offset * 2;
        let mid_pos = pos as isize + offset;
        if new_pos >= 0 && new_pos < 64 && piece::is_empty(board.get_square(new_pos as usize)) && piece::is_empty(board.get_square(mid_pos as usize)) {
            moves.push(Move::new_with_flags(pos, new_pos as usize, DOUBLE_PAWN_PUSH));
        }
    }

    let attack_offsets: [isize; 2] = if piece::get_color(pawn) == piece::WHITE { [-9, -7] } else { [9, 7] };
    for offset in attack_offsets {
        let new_pos = pos as isize + offset;
        if new_pos < 0 || new_pos >= 64 {
            continue;
        }
        if (new_pos as usize % 8).abs_diff(pos % 8) != 1 {
            continue;
        }
        if !piece::is_empty(board.get_square(new_pos as usize)) && piece::get_color(board.get_square(new_pos as usize)) != piece::get_color(pawn) {
            if new_pos / 8 == 7 || new_pos / 8 == 0 {
                moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION | CAPTURE));
                moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION | CAPTURE).set_promotion_piece(piece::ROOK));
                moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION | CAPTURE).set_promotion_piece(piece::BISHOP));
                moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION | CAPTURE).set_promotion_piece(piece::KNIGHT));

                // moves.push(Move::new_with_flags(pos, new_pos as usize, PROMOTION | CAPTURE).set_promotion_piece(board.get_promotion_piece()));

            } else {
                moves.push(Move::new_with_flags(pos, new_pos as usize, CAPTURE));
            }
        } else if new_pos as usize == board.get_en_passant_square() {
            moves.push(Move::new_with_flags(pos, new_pos as usize, EN_PASSANT | CAPTURE));
        } 
    }
}

pub fn get_sliding_moves(board: &Board, pos: usize, get_straights: bool, get_diagonals: bool, moves: &mut Vec<Move>) {
    if !get_straights && !get_diagonals {
        return;
    }
    let piece = board.get_square(pos);

    let direction_offsets: [isize; 8] = [
        -1, 1, -8, 8, -9, -7, 7, 9
    ];
    let to_edges: [usize; 4] = [
        pos % 8, //left
        7 - pos % 8, //right
        pos / 8, //top
        7 - pos / 8, //bottom
    ];
    let length_to_check: [usize; 8] = [
        to_edges[0],
        to_edges[1],
        to_edges[2],
        to_edges[3],
        to_edges[0].min(to_edges[2]),
        to_edges[1].min(to_edges[2]),
        to_edges[0].min(to_edges[3]),
        to_edges[1].min(to_edges[3])
    ];

    for i in 0..length_to_check.len() {
        if i < 4 && !get_straights {
            continue;
        }
        if i >= 4 && !get_diagonals {
            continue;
        }
        for n in 0..length_to_check[i] {
            let new_pos = pos as isize + direction_offsets[i] * ((n + 1) as isize); // Shoutout to Sebastian Lague for this formula. https://youtu.be/U4ogK0MIzqk?is=5VpqB7vtvDza2phT
            if new_pos < 0 || new_pos >= 64 {
                break;
            }
            if piece::get_color(board.get_square(new_pos as usize)) == piece::get_color(piece) {
                break;
            }
            if piece::get_color(board.get_square(new_pos as usize)) != piece::get_color(piece) && !piece::is_empty(board.get_square(new_pos as usize)) {
                moves.push(Move::new_with_flags(pos, new_pos as usize, CAPTURE));
                break;
            }
            moves.push(Move::new(pos, new_pos as usize));
        }
    }
}