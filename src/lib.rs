mod board;
mod piece;
mod r#move;

use board::Board;
use r#move::Move;

#[derive(Copy, Clone)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
    Draw,
}
pub struct Game {
    board: Board,
    state: GameState,
    history: Vec<(Board, GameState)>,
}

impl Game {
    /** Create a new game. */
    pub fn new() -> Self {
        Self {
            board: Board::create_default_position(),
            state: GameState::InProgress,
            history: Vec::new(),
        }
    }

    pub fn new_from_fen(fen: &str) -> Self {
        Self {
            board: Board::create_from_fen(fen),
            state: GameState::InProgress,
            history: Vec::new(),
        }
    }

    /* 
     * Set what piece will be promoted to.
     * @param piece: The piece to set the promotion piece to. 'Q' for queen, 'R' for rook, 'B' for bishop, 'N' for knight.
     */
    pub fn set_promotion(&mut self, piece: String) {
        let promotion_piece = match piece.chars().nth(0) {
            Some(piece) => {
                if piece == 'Q' {
                    piece::QUEEN
                }
                else if piece == 'R' {
                    piece::ROOK
                }
                else if piece == 'B' {
                    piece::BISHOP
                }
                else if piece == 'N' {
                    piece::KNIGHT
                }
                else {
                    piece::QUEEN
                }
            },
            None => return,
        };
        self.board.set_promotion_piece(promotion_piece);
    }

    /** Get the game state. */
    pub fn get_game_state(&self) -> GameState { self.state }
    
    /*
     * Set the game state.
     * @param state: The game state to set.
     */
    pub fn set_game_state(&mut self, state: GameState) { self.state = state; }

    /* 
    * Get the board as a 2D array of characters.
    * @return: A 2D array of characters representing the board.
    *          The characters are:
    *          - 'P' for white pawn
    *          - 'R' for white rook
    *          - 'N' for white knight
    *          - 'B' for white bishop
    *          - 'Q' for white queen
    *          - 'K' for white king
    *          - 'p' for black pawn
    *          - 'r' for black rook
    *          - 'n' for black knight
    *          - 'b' for black bishop
    *          - 'q' for black queen
    *          - 'k' for black king
    *          - '.' for empty square
    */
    pub fn get_board(&self) -> [[char; 8]; 8] {
        let mut board = [[0 as char; 8]; 8];
        for i in 0..64 {
            let piece = self.board.get_square(i);
            let file = i % 8;
            let rank = i / 8;
            board[rank][file] = match piece {
                piece::PAWN => { if piece == piece::WHITE { 'P' } else { 'p' } },
                piece::ROOK => { if piece == piece::WHITE { 'R' } else { 'r' } },
                piece::KNIGHT => { if piece == piece::WHITE { 'N' } else { 'n' } },
                piece::BISHOP => { if piece == piece::WHITE { 'B' } else { 'b' } },
                piece::QUEEN => { if piece == piece::WHITE { 'Q' } else { 'q' } },
                piece::KING => { if piece == piece::WHITE { 'K' } else { 'k' } },
                _ => '.',
            }
        }
        board
    }
    /*
     * Get the color of the current turn.
     * @return: "white" if the current turn is white, "black" if the current turn is black.
     */
    pub fn get_turn_color(&self) -> String { if self.board.current_color_turn == piece::WHITE { String::from("white") } else { String::from("black") } }
    /*
     * Get the promotion piece.
     * @return: The promotion piece.
     *          - "queen" for queen
     *          - "rook" for rook
     *          - "bishop" for bishop
     *          - "knight" for knight
     */
    pub fn get_promotion_piece(&self) -> String { 
        match self.board.promotion_piece { 
            piece::QUEEN => String::from("queen"),
            piece::ROOK => String::from("rook"),
            piece::BISHOP => String::from("bishop"),
            piece::KNIGHT => String::from("knight"),
            _ => String::from("queen"),
        }
    }
    /*
     * Get the halfmove clock.
     * @return: The halfmove clock as a u32.
     */
    pub fn get_halfmove(&self) -> u32 { self.board.halfmove }
    /*
     * Get the fullmove clock.
     * @return: The fullmove clock as a u32.
     */
    pub fn get_fullmove(&self) -> u32 { self.board.fullmove }

    /* 
    * Get the possible moves for a given piece square.
    * @param piece_square: The square of the piece to get the possible moves for. (e.g. "e4")
    * @return: A vector of possible moves.
    */
    pub fn get_possible_moves(&self, piece_square: String) -> Option<Vec<String>> { 
        let piece_file = match piece_square.chars().nth(0) {
            Some(file) => {
                if file < 'a' || file > 'h' {
                    return None;
                }
                file as usize - 'a' as usize
            },
            None => return None,
        };
        let piece_rank = match piece_square.chars().nth(1) {
            Some(rank) => {
                if rank < '1' || rank > '8' {
                    return None;
                }
                rank as usize - '1' as usize
            },
            None => return None,
        };

        let legal_moves = match self.board.get_legal_moves() {
            Some(moves) => moves,
            None => return None,
        };

        let possible_moves = legal_moves.iter().filter(
            |m| m.from == piece_rank * 8 + piece_file
        ).map(|m| format!("{}", m)).collect();

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

    /* 
    * Make a move on the board.
    * @param from: The square to move from. (e.g. "e4")
    * @param to: The square to move to. (e.g. "e5")
    * @return: The new game state.
    */
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> { 
        let from_file = match from.chars().nth(0) {
            Some(file) => {
                if file < 'a' || file > 'h' {
                    return None;
                }
                file as usize - 'a' as usize
            },
            None => return None,
        };
        let from_rank = match from.chars().nth(1) {
            Some(rank) => {
                if rank < '1' || rank > '8' {
                    return None;
                }
                rank as usize - '1' as usize
            },
            None => return None,
        };
        let to_file = match to.chars().nth(0) {
            Some(file) => {
                if file < 'a' || file > 'h' {
                    return None;
                }
                file as usize - 'a' as usize
            },
            None => return None,
        };
        let to_rank = match to.chars().nth(1) {
            Some(rank) => {
                if rank < '1' || rank > '8' {
                    return None;
                }
                rank as usize - '1' as usize
            },
            None => return None,
        };

        let legal_moves = match self.board.get_legal_moves() {
            Some(moves) => moves,
            None => return None,
        };
        let possible_moves: Vec<&Move> = legal_moves.iter().filter(
            |m| m.from == from_rank * 8 + from_file && m.to == to_rank * 8 + to_file
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

        if self.board.is_in_check(None) { return Some(GameState::Check); }
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
        return Some(GameState::InProgress);
    }

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

    fn test() {
        let mut game = Game::new();
        let new_state = match game.make_move(String::from("e2"), String::from("e4")) {
            Some(state) => state,
            None => game.get_game_state(),
        };
        game.set_game_state(new_state);
    }
}