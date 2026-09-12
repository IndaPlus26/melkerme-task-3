mod board;
mod piece;
mod r#move;

use board::Board;
use r#move::Move;

/// Enum for the game state.
/// NOTE!!! The game state is updated **after** a move is made.
/// For example, if a move is made and it results in a check, the game state will be updated to Checked.
#[derive(Copy, Clone)]
pub enum GameState {
    InProgress,
    Quiet,
    Captured,
    Checked,
    Promoted,
    Castled,
    Checkmate,
    Stalemate,
    Draw,
}

/// A color enum
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

/// A piece enum
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// The Game struct
pub struct Game {
    board: Board,
    state: GameState,
    history: Vec<(Board, GameState)>,
}

impl Game {
    /// Create a new game from the default starting position
    pub fn new() -> Self {
        Self {
            board: Board::create_default_position(),
            state: GameState::InProgress,
            history: Vec::new(),
        }
    }

    /// Create a new game from a FEN string
    pub fn new_from_fen(fen: &str) -> Self {
        Self {
            board: Board::create_from_fen(fen),
            state: GameState::InProgress,
            history: Vec::new(),
        }
    }

    /// Get the game state.
    pub fn get_game_state(&self) -> GameState { self.state }
    
    /**
     * Set the game state.
     * @param state: The game state to set.
     */
    pub fn set_game_state(&mut self, state: GameState) { self.state = state; }

    /// Get the current board state as a FEN string.
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty_count = 0;
        for (i, square) in self.board.squares.iter().enumerate() {
            let piece = piece::get_piece(*square);
            let color = piece::get_color(*square);
            if piece == 0 {
                empty_count += 1;
            } else {
                if empty_count > 0 {
                    fen += &format!("{}", empty_count);
                    empty_count = 0;
                }
                fen += match piece {
                    piece::PAWN => { if color == piece::WHITE { "P" } else { "p" } },
                    piece::ROOK => { if color == piece::WHITE { "R" } else { "r" } },
                    piece::KNIGHT => { if color == piece::WHITE { "N" } else { "n" } },
                    piece::BISHOP => { if color == piece::WHITE { "B" } else { "b" } },
                    piece::QUEEN => { if color == piece::WHITE { "Q" } else { "q" } },
                    piece::KING => { if color == piece::WHITE { "K" } else { "k" } },
                    _ => "",
                };
            }
            if i % 8 == 7 {
                if empty_count > 0 {
                    fen += format!("{}", empty_count).as_str();
                    empty_count = 0;
                }
                if i < 63 {
                    fen += "/";
                }
            }
        }
        fen += if self.board.current_color_turn == piece::WHITE { " w " } else { " b " };
        if self.board.get_castling_rights(0) {
            fen += "K";
        }
        if self.board.get_castling_rights(1) {
            fen += "Q";
        }
        if self.board.get_castling_rights(2) {
            fen += "k";
        }
        if self.board.get_castling_rights(3) {
            fen += "q";
        }

        if self.board.get_en_passant_square() != 64 {
            fen += &format!(" {}", Board::from_index_to_square(self.board.get_en_passant_square()));
        } else {
            fen += " -";
        }

        fen += &format!(" {}", self.board.halfmove);
        fen += &format!(" {}", self.board.fullmove);

        fen
    }
    
    /**
     * Get the color of the current turn.
     * @return: Color::White if the current turn is white, Color::Black if the current turn is black.
     */
    pub fn get_turn_color(&self) -> Color { if self.board.current_color_turn == piece::WHITE { Color::White } else { Color::Black } }
    
    /**
     * Get the current promotion piece.
     * @return: The promotion piece as a Piece enum.
     */
    pub fn get_promotion_piece(&self) -> Piece { 
        match self.board.promotion_piece { 
            piece::QUEEN => Piece::Queen,
            piece::ROOK => Piece::Rook,
            piece::BISHOP => Piece::Bishop,
            piece::KNIGHT => Piece::Knight,
            _ => Piece::Queen,
        }
    }

    /**
     * Set what piece will be promoted to.
     * @param piece: The piece to set the promotion piece to. Use the Piece enum.
     * 
     * NOTE!!! This will have to be set before the move is played.
     * Make the user set the promotion piece before calling make_move().
     * The defualt is queen. So if you can't be bothered to implement this, all promotions will become queens.
     */
    pub fn set_promotion(&mut self, piece: Piece) {
        let promotion_piece = match piece {
            Piece::Queen => piece::QUEEN,
            Piece::Rook => piece::ROOK,
            Piece::Knight => piece::KNIGHT,
            Piece::Bishop => piece::BISHOP,
            Piece::King => piece::KING,
            _ => piece::QUEEN,
        };
        self.board.set_promotion_piece(promotion_piece);
    }

    // Get the halfmove clock/counter
    pub fn get_halfmove(&self) -> u32 { self.board.halfmove }

    // Get the fullmove clock/counter
    pub fn get_fullmove(&self) -> u32 { self.board.fullmove }

    /**
    * Get the possible moves from a given square.
    * @param piece_square: The `from` square of the piece to get the possible moves for. (e.g. "E4")
    * @return: A vector of possible `to` squares as strings.
    * example: get_possible_moves("E2") -> Some(vec!["E3", "E4"])
    * example: get_possible_moves("E2") -> None (if there are no possible moves)
    */
    pub fn get_possible_moves(&self, piece_square: String) -> Option<Vec<String>> { 
        let piece_file = match piece_square.chars().nth(0) {
            Some(file) => {
                if file < 'A' || file > 'H' {
                    return None;
                }
                file as usize - 'A' as usize
            },
            None => return None,
        };
        let piece_rank = match piece_square.chars().nth(1) {
            Some(rank) => {
                if rank < '1' || rank > '8' {
                    return None;
                }
                7 - (rank as usize - '1' as usize)
            },
            None => return None,
        };

        let legal_moves = match self.board.get_legal_moves() {
            Some(moves) => moves,
            None => return None,
        };

        let possible_moves = legal_moves.iter()
        .filter(|m| m.from == piece_rank * 8 + piece_file)
        .map(|m| format!("{}", m.to_string().split_off(2))).collect::<Vec<String>>();

        if possible_moves.len() < 1 {
            return None;
        }

        return Some(possible_moves);
    }

    fn check_draw(&mut self) -> bool {
        let mut repetition_count = 0;
        for i in 0..self.history.len() {
            if self.history[i].0.squares == self.board.squares {
                repetition_count += 1;
                if repetition_count >= 3 {
                    return true;
                }
            }
        }
        if repetition_count >= 3 {
            return true;
        }
        if self.board.halfmove >= 50 {
            return true;
        }
        false
    }

    /**
    * Make a move on the board.
    * @param from: The square to move from. (e.g. "E4")
    * @param to: The square to move to. (e.g. "E5")
    * @return: The new game state.
    * NOTE!!! The new game state is not set automatically, you must set it manually after calling this function.
    */
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let from_index = Board::from_square_to_index(&from);
        let to_index = Board::from_square_to_index(&to);

        let legal_moves = match self.board.get_legal_moves() {
            Some(moves) => moves,
            None => return None,
        };
        let possible_moves: Vec<&Move> = legal_moves.iter().filter(
            |m| m.from == from_index && m.to == to_index
        ).collect();
        
        let move_played: &Move;
        if possible_moves.len() < 1 {
            return None;
        } else if possible_moves.len() > 1 {
            let promotion_piece = self.board.get_promotion_piece();
            move_played = possible_moves.into_iter().find(|m| m.get_promotion_piece() == promotion_piece).unwrap();
        } else {
            move_played = possible_moves.into_iter().next().unwrap();
        }

        self.history.push((self.board, self.state));
        self.board.make_move(*move_played);

        let mut new_state = GameState::Quiet;

        if self.board.is_in_check(None) { new_state = GameState::Checked; }
        if move_played.is_capture() { new_state = GameState::Captured; }
        if move_played.is_promotion() { new_state = GameState::Promoted; }
        if move_played.is_castling() { new_state = GameState::Castled; }

        let has_legal_moves = match self.board.get_legal_moves() {
            Some(moves) => moves.len() > 0,
            None => false,
        };
        if !has_legal_moves {
            if self.board.is_in_check(None) { return Some(GameState::Checkmate); }
            else { return Some(GameState::Stalemate); }
        }
        if self.check_draw() {
            return Some(GameState::Draw);
        }
        return Some(new_state);
    }

    /// All moves made are stored in a vector.
    /// This function will undo the last move made.
    pub fn undo_move(&mut self) {
        let Some((board, state)) = self.history.pop() else { return };
        self.board = board;
        self.state = state;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_legal_move_count() {
        let cases = [
            (
                Game::new(),
                [20, 400, 8902, 197281, 4865609],
            ),
            (
                Game::new_from_fen(
                    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
                ),
                [48, 2039, 97862, 4085603, 193690690],
            ),
            (
                Game::new_from_fen(
                    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
                ),
                [14, 191, 2812, 43238, 674624],
            ),
            (
                Game::new_from_fen(
                    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                ),
                [6, 264, 9467, 422333, 15833292],
            ),
            (
                Game::new_from_fen(
                    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                ),
                [44, 1486, 62379, 2103487, 89941194],
            ),
        ];

        // for every case create 5 nested loops for a depth of 5
        for (case_index, (game, expected)) in cases.into_iter().enumerate() {
            let mut counts = [0usize; 5];
            let board = game.board;

            for m1 in board.get_legal_moves().into_iter().flatten() {
                counts[0] += 1;
                let mut b1 = board;
                b1.make_move(m1);

                for m2 in b1.get_legal_moves().into_iter().flatten() {
                    counts[1] += 1;
                    let mut b2 = b1;
                    b2.make_move(m2);

                    for m3 in b2.get_legal_moves().into_iter().flatten() {
                        counts[2] += 1;
                        let mut b3 = b2;
                        b3.make_move(m3);

                        for m4 in b3.get_legal_moves().into_iter().flatten() {
                            counts[3] += 1;
                            let mut b4 = b3;
                            b4.make_move(m4);

                            for _m5 in b4.get_legal_moves().into_iter().flatten() {
                                counts[4] += 1;
                            }
                        }
                    }
                }
            }

            for (index, (actual, expected)) in
                counts.into_iter().zip(expected).enumerate()
            {
                assert_eq!(
                    actual,
                    expected,
                    "Case {}, depth {}",
                    case_index + 1,
                    index + 1,
                );
            }
        }
    }

    #[test]
    fn test_from_index_to_square() {
        assert_eq!(Board::from_index_to_square(0), "A8");
        assert_eq!(Board::from_index_to_square(1), "B8");
        assert_eq!(Board::from_index_to_square(63), "H1");
    }

    #[test]
    fn test_from_square_to_index() {
        assert_eq!(Board::from_square_to_index("A8"), 0);
        assert_eq!(Board::from_square_to_index("B8"), 1);
        assert_eq!(Board::from_square_to_index("H1"), 63);
    }

    #[test]
    fn test_to_fen() {
        let game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        assert_eq!(game.to_fen(), "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    }
}