use crate::r#move::{Move, get_king_moves, get_knight_moves, get_pawn_moves, get_sliding_moves};
use crate::piece;

pub const CASTLING_RIGHTS_WHITE_KING_SIDE: usize = 0;
pub const CASTLING_RIGHTS_WHITE_QUEEN_SIDE: usize = 1;
pub const CASTLING_RIGHTS_BLACK_KING_SIDE: usize = 2;
pub const CASTLING_RIGHTS_BLACK_QUEEN_SIDE: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub squares: [u8; 64], // A8 = 0 top-left, H1 = 63 bottom-right, basically like the board is viewed from white's perspective
    pub current_color_turn: u8,
    castling_rights: [bool; 4],
    en_passant_square: usize, // 64 if no en passant square, otherwise the square of the pawn that can be captured en passant
    pub halfmove: u32,
    pub fullmove: u32,
    pub promotion_piece: u8,
}

impl Board {
    fn new() -> Self {
        Self {
            squares: [0; 64],
            current_color_turn: piece::WHITE,
            castling_rights: [true, true, true, true],
            en_passant_square: 0,
            halfmove: 0,
            fullmove: 0,
            promotion_piece: piece::QUEEN,
        }
    }
    pub fn create_from_fen(fen: &str) -> Self {
        let mut board = Self::new();
        let mut fen_iter = fen.split_whitespace();

        // Get the board state as char vec
        let board_state = match fen_iter.next() {
            Some(board_state) => board_state.chars(),
            None => panic!("Invalid FEN string"),
        };

        // Get the white turn
        board.current_color_turn = match fen_iter.next() {
            Some(white_move) => {
                if white_move == "w" {
                    piece::WHITE
                } else {
                    piece::BLACK
                }
            }
            None => panic!("Invalid FEN string"),
        };

        // Get the castling rights
        board.castling_rights = match fen_iter.next() {
            Some(castling_rights) => {
                let mut rights = [false; 4];
                for c in castling_rights.chars() {
                    match c {
                        'K' => rights[CASTLING_RIGHTS_WHITE_KING_SIDE] = true,
                        'Q' => rights[CASTLING_RIGHTS_WHITE_QUEEN_SIDE] = true,
                        'k' => rights[CASTLING_RIGHTS_BLACK_KING_SIDE] = true,
                        'q' => rights[CASTLING_RIGHTS_BLACK_QUEEN_SIDE] = true,
                        _ => continue,
                    }
                }
                rights
            }
            None => panic!("Invalid FEN string"),
        };

        // Get the en passant square
        board.en_passant_square = match fen_iter.next() {
            Some(en_passant_square) => match en_passant_square {
                "-" => 64,
                _ => {
                    let file = en_passant_square.chars().nth(0).unwrap() as usize - b'a' as usize;
                    let rank = en_passant_square.chars().nth(1).unwrap() as usize - b'1' as usize;
                    file + (7 - rank) * 8
                }
            },
            None => panic!("Invalid FEN string"),
        };

        // Get the halfmove count
        board.halfmove = match fen_iter.next() {
            Some(halfmove) => halfmove.parse().unwrap(),
            None => 0,
        };

        // Get the fullmove count
        board.fullmove = match fen_iter.next() {
            Some(fullmove) => fullmove.parse().unwrap(),
            None => 1,
        };

        // Parse and set the board state
        let mut i = 0;
        for c in board_state {
            match c {
                'P' => board.set_square(i, piece::create(piece::PAWN, piece::WHITE)),
                'R' => board.set_square(i, piece::create(piece::ROOK, piece::WHITE)),
                'N' => board.set_square(i, piece::create(piece::KNIGHT, piece::WHITE)),
                'B' => board.set_square(i, piece::create(piece::BISHOP, piece::WHITE)),
                'Q' => board.set_square(i, piece::create(piece::QUEEN, piece::WHITE)),
                'K' => board.set_square(i, piece::create(piece::KING, piece::WHITE)),
                'p' => board.set_square(i, piece::create(piece::PAWN, piece::BLACK)),
                'r' => board.set_square(i, piece::create(piece::ROOK, piece::BLACK)),
                'n' => board.set_square(i, piece::create(piece::KNIGHT, piece::BLACK)),
                'b' => board.set_square(i, piece::create(piece::BISHOP, piece::BLACK)),
                'q' => board.set_square(i, piece::create(piece::QUEEN, piece::BLACK)),
                'k' => board.set_square(i, piece::create(piece::KING, piece::BLACK)),
                '/' => continue,
                num => i += (num as u8 - b'1') as usize,
            }
            i += 1;
        }

        board
    }
    pub fn create_default_position() -> Self {
        Self::create_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn set_square(&mut self, square: usize, piece: u8) {
        self.squares[square] = piece;
    }
    pub fn get_square(&self, square: usize) -> u8 {
        self.squares[square]
    }
    pub fn set_promotion_piece(&mut self, piece: u8) {
        self.promotion_piece = piece;
    }
    pub fn get_promotion_piece(&self) -> u8 {
        self.promotion_piece
    }
    pub fn get_castling_rights(&self, index: usize) -> bool {
        self.castling_rights[index]
    }
    pub fn set_castling_rights(&mut self, index: usize, value: bool) {
        self.castling_rights[index] = value;
    }
    pub fn set_en_passant_square(&mut self, square: usize) {
        self.en_passant_square = square;
    }
    pub fn get_en_passant_square(&self) -> usize {
        self.en_passant_square
    }
    pub fn swap_current_turn_color(&mut self) {
        self.current_color_turn = if self.current_color_turn == piece::WHITE {
            piece::BLACK
        } else {
            piece::WHITE
        };
    }

    pub fn is_in_check(&self, color: Option<u8>) -> bool {
        // If no color is provided, use the current turn color
        let defending_color = match color {
            Some(color) => color,
            None => self.current_color_turn,
        };
        let attacking_color = if defending_color == piece::WHITE {
            piece::BLACK
        } else {
            piece::WHITE
        };

        for i in 0..64 {
            // Get king pos and check if attacked
            if self.get_square(i) == piece::create(piece::KING, defending_color) {
                return self.is_square_controlled(i, attacking_color);
            }
        }
        false
    }
    fn is_square_controlled(&self, square: usize, attacking_color: u8) -> bool {
        // Move generation in reverse

        // Pawn attacks
        let pawn_offsets = if attacking_color == piece::WHITE {
            [7, 9]
        } else {
            [-7, -9]
        };
        for offset in pawn_offsets {
            let new_square = square as isize + offset;
            if new_square >= 0
                && new_square < 64
                && (new_square as usize % 8).abs_diff(square % 8) == 1
            {
                if self.get_square(new_square as usize)
                    == piece::create(piece::PAWN, attacking_color)
                {
                    return true;
                }
            }
        }

        // Knight attacks
        let knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17];
        for offset in knight_offsets {
            let new_square = square as isize + offset;
            if new_square >= 0
                && new_square < 64
                && (new_square as usize % 8).abs_diff(square % 8) <= 2
                && (new_square as usize / 8).abs_diff(square / 8) <= 2
            {
                if self.get_square(new_square as usize)
                    == piece::create(piece::KNIGHT, attacking_color)
                {
                    return true;
                }
            }
        }

        // Sliding piece attacks
        let direction_offsets: [isize; 8] = [
            -1, // Left
            1,  // Right
            -8, // Up
            8,  // Down
            -9, // Up-left
            -7, // Up-right
            7,  // Down-left
            9,  // Down-right
        ];
        let to_edges = [
            square % 8,       // Left edge
            7 - (square % 8), // Right edge
            square / 8,       // Up edge
            7 - (square / 8), // Down edge
        ];
        let length_to_edges = [
            to_edges[0],                  // Left edge
            to_edges[1],                  // Right edge
            to_edges[2],                  // Up edge
            to_edges[3],                  // Down edge
            to_edges[0].min(to_edges[2]), // Up-left edge
            to_edges[1].min(to_edges[2]), // Up-right edge
            to_edges[0].min(to_edges[3]), // Down-left edge
            to_edges[1].min(to_edges[3]), // Down-right edge
        ];
        for dir_index in 0..8 {
            for n in 0..length_to_edges[dir_index] {
                // This feels like magic but makes sense if you think about it
                let new_square = square as isize + direction_offsets[dir_index] * (n + 1) as isize;

                // Check if the new square is on the board
                if new_square >= 0 && new_square < 64 {
                    let attacking_piece = self.get_square(new_square as usize);
                    if piece::is_empty(attacking_piece) {
                        continue; // This direction is not blocked by anything yet, keep checking
                    }
                    if piece::get_color(attacking_piece) != attacking_color {
                        break; // Line of sight broken by friendly piece, exit
                    }

                    let attacking_piece = piece::get_piece(attacking_piece);
                    if attacking_piece == piece::QUEEN {
                        return true;
                    }
                    if attacking_piece == piece::ROOK && dir_index < 4 {
                        return true;
                    } // Striaghts are the first 4 directions
                    if attacking_piece == piece::BISHOP && dir_index >= 4 {
                        return true;
                    } // Diagonals are the last 4 directions
                    if attacking_piece == piece::KING && n == 0 {
                        return true;
                    } // King controls the closest square in every direction
                    break;
                }
            }
        }

        false
    }
    fn get_pseudo_legal_moves(&self) -> Option<Vec<Move>> {
        let mut moves: Vec<Move> = Vec::with_capacity(64);
        for i in 0..64 {
            if piece::get_color(self.get_square(i)) != self.current_color_turn {
                continue;
            }

            // Get Queen moves
            if piece::get_piece(self.get_square(i)) == piece::QUEEN {
                get_sliding_moves(self, i, true, true, &mut moves);
            }

            // Get Rook moves
            if piece::get_piece(self.get_square(i)) == piece::ROOK {
                get_sliding_moves(self, i, true, false, &mut moves);
            }

            // Get Bishop moves
            if piece::get_piece(self.get_square(i)) == piece::BISHOP {
                get_sliding_moves(self, i, false, true, &mut moves);
            }

            // Get knight moves
            if piece::get_piece(self.get_square(i)) == piece::KNIGHT {
                get_knight_moves(self, i, &mut moves);
            }

            // Get king moves
            if piece::get_piece(self.get_square(i)) == piece::KING {
                get_king_moves(self, i, &mut moves);
            }

            // Get pawn moves
            if piece::get_piece(self.get_square(i)) == piece::PAWN {
                get_pawn_moves(self, i, &mut moves);
            }
        }
        Some(moves)
    }
    pub fn get_legal_moves(&self, color: Option<u8>) -> Option<Vec<Move>> {
        let mut new_board = *self;
        if color.is_some() && self.current_color_turn != color.unwrap() {
            new_board.swap_current_turn_color();
        }

        let pseudo_legal_moves = new_board.get_pseudo_legal_moves();
        if pseudo_legal_moves.is_none() {
            return None;
        }
        let mut legal_moves: Vec<Move> = Vec::new();

        for m in pseudo_legal_moves.unwrap() {
            // More castling rules plus double checks
            if m.is_castling() {
                if m.to == 62 {
                    if !new_board.get_castling_rights(CASTLING_RIGHTS_WHITE_KING_SIDE) {
                        continue;
                    }
                    if new_board.get_square(63) != piece::create(piece::ROOK, piece::WHITE) {
                        continue;
                    }
                    if !piece::is_empty(new_board.get_square(61))
                        || !piece::is_empty(new_board.get_square(62))
                    {
                        continue;
                    }
                    if new_board.is_square_controlled(61, piece::BLACK)
                        || new_board.is_square_controlled(60, piece::BLACK)
                    {
                        continue;
                    }
                } else if m.to == 58 {
                    if !new_board.get_castling_rights(CASTLING_RIGHTS_WHITE_QUEEN_SIDE) {
                        continue;
                    }
                    if new_board.get_square(56) != piece::create(piece::ROOK, piece::WHITE) {
                        continue;
                    }
                    if !piece::is_empty(new_board.get_square(59))
                        || !piece::is_empty(new_board.get_square(58))
                        || !piece::is_empty(new_board.get_square(57))
                    {
                        continue;
                    }
                    if new_board.is_square_controlled(59, piece::BLACK)
                        || new_board.is_square_controlled(60, piece::BLACK)
                    {
                        continue;
                    }
                } else if m.to == 6 {
                    if !new_board.get_castling_rights(CASTLING_RIGHTS_BLACK_KING_SIDE) {
                        continue;
                    }
                    if new_board.get_square(7) != piece::create(piece::ROOK, piece::BLACK) {
                        continue;
                    }
                    if !piece::is_empty(new_board.get_square(5))
                        || !piece::is_empty(new_board.get_square(6))
                    {
                        continue;
                    }
                    if new_board.is_square_controlled(5, piece::WHITE)
                        || new_board.is_square_controlled(4, piece::WHITE)
                    {
                        continue;
                    }
                } else if m.to == 2 {
                    if !new_board.get_castling_rights(CASTLING_RIGHTS_BLACK_QUEEN_SIDE) {
                        continue;
                    }
                    if new_board.get_square(0) != piece::create(piece::ROOK, piece::BLACK) {
                        continue;
                    }
                    if !piece::is_empty(new_board.get_square(3))
                        || !piece::is_empty(new_board.get_square(2))
                        || !piece::is_empty(new_board.get_square(1))
                    {
                        continue;
                    }
                    if new_board.is_square_controlled(3, piece::WHITE)
                        || new_board.is_square_controlled(4, piece::WHITE)
                    {
                        continue;
                    }
                }
            }

            // General test for a check, this is could be improved but it's fast enough for now
            new_board.make_move(m);
            if !new_board.is_in_check(Some(new_board.current_color_turn)) {
                legal_moves.push(m);
            }
        }

        if legal_moves.is_empty() {
            return None;
        }
        Some(legal_moves)
    }
    pub fn make_move(&mut self, move_played: Move) {
        self.set_square(move_played.to, self.get_square(move_played.from));
        self.set_square(move_played.from, 0);

        self.swap_current_turn_color();

        self.set_en_passant_square(64);
        piece::post_move_update(self, move_played);
    }

    pub fn from_index_to_square(index: usize) -> String {
        let file = index % 8;
        let rank = index / 8;
        format!(
            "{}{}",
            (file as u8 + b'A') as char,
            (8 - rank as u8 + b'0') as char
        )
    }
    pub fn from_square_to_index(square: &str) -> usize {
        let file = square.chars().nth(0).unwrap() as usize - b'A' as usize;
        let rank = square.chars().nth(1).unwrap() as usize - b'0' as usize;
        file + (8 - rank) * 8
    }
}
