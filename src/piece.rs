use crate::board::Board;
use crate::r#move::Move;

pub const KING: u8 =   0b00000001;
pub const QUEEN: u8 =  0b00000010;
pub const ROOK: u8 =   0b00000100;
pub const BISHOP: u8 = 0b00001000;
pub const KNIGHT: u8 = 0b00010000;
pub const PAWN: u8 =   0b00100000;
pub const WHITE: u8 =  0b01000000;
pub const BLACK: u8 =  0b10000000;

// Store color in the first 2 bits and the piece in the last 6 bits
// Empty squares have no color or piece, so they are 0
pub fn create(piece: u8, color: u8) -> u8 {
    piece | color
}

pub fn get_color(piece: u8) -> u8 {
    piece & (WHITE | BLACK) // 0b11000000 -> 0b01000000 or 0b10000000 or 0
}

pub fn get_piece(piece: u8) -> u8 {
    piece & (KING | QUEEN | ROOK | BISHOP | KNIGHT | PAWN) // 0b00111111 -> 0b00000001 or 0b00000010 or 0b00000100 or 0b00001000 or 0b00010000 or 0b00100000 or 0
}

pub fn is_empty(piece: u8) -> bool {
    piece == 0
}

pub fn post_move_update(board: &mut Board, move_played: Move) {
    let piece = board.get_square(move_played.to); // This is called after the piece has already been moved to the new position
    if is_empty(piece) {
        return;
    }

    if get_piece(piece) == KING {
        // Can't castle if the king has moved
        if get_color(piece) == WHITE {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_KING_SIDE, false);
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_QUEEN_SIDE, false);
        } else {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_KING_SIDE, false);
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_QUEEN_SIDE, false);
        }
        // Move the rook when castling
        if move_played.is_castling() {
            // King side castling
            if move_played.to == 62 {
                board.set_square(61, create(ROOK, get_color(piece)));
                board.set_square(63, 0);
            } else if move_played.to == 58 {
                board.set_square(59, create(ROOK, get_color(piece)));
                board.set_square(56, 0);
            } else if move_played.to == 6 {
                board.set_square(5, create(ROOK, get_color(piece)));
                board.set_square(7, 0);
            } else if move_played.to == 2 {
                board.set_square(3, create(ROOK, get_color(piece)));
                board.set_square(0, 0);
            }
        }
    }

    if get_piece(piece) == ROOK {
        // Can't castle if the rook has moved
        if move_played.from == 0 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_QUEEN_SIDE, false);
        }
        if move_played.from == 7 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_KING_SIDE, false);
        }
        if move_played.from == 56 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_QUEEN_SIDE, false);
        }
        if move_played.from == 63 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_KING_SIDE, false);
        }
    }

    if get_piece(piece) == PAWN {
        // Remove en passanted pawn
        if move_played.is_en_passant() {
            let opponent_pawn_pos = move_played.to as isize + if get_color(piece) == WHITE { 8 } else { -8 } as isize;
            if opponent_pawn_pos < 0 || opponent_pawn_pos >= 64 {
                return;
            }
            board.set_square(opponent_pawn_pos as usize, 0);
        }
        // Set en passant square
        if move_played.is_double_pawn_push() {
            let en_passant_square = move_played.to as isize + if get_color(piece) == WHITE { 8 } else { -8 } as isize;
            if en_passant_square < 0 || en_passant_square >= 64 {
                return;
            }
            board.set_en_passant_square(en_passant_square as usize);
        }
        // Replace pawn with promotion piece
        if move_played.is_promotion() {
            board.set_square(move_played.to, create(move_played.get_promotion_piece(), get_color(piece)));
        }
    }

    if move_played.is_capture() {
        // Can't castle if a rook has been captured
        if move_played.to == 0 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_QUEEN_SIDE, false);
        }
        if move_played.to == 7 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_KING_SIDE, false);
        }
        if move_played.to == 56 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_WHITE_QUEEN_SIDE, false);
        }
        if move_played.to == 63 {
            board.set_castling_rights(crate::board::CASTLING_RIGHTS_BLACK_KING_SIDE, false);
        }
    }
}