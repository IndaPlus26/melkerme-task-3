use std::collections::HashMap;

use crate::board::Board;
use crate::piece;
use crate::r#move::Move;

fn evaluate(board: &Board) -> f32 {
    let mut score = 0.0;

    let piece_values: HashMap<u8, f32> = HashMap::from([
        (piece::PAWN, 1.0),
        (piece::KNIGHT, 3.0),
        (piece::BISHOP, 3.1),
        (piece::ROOK, 5.0),
        (piece::QUEEN, 9.0),
    ]);

    for i in 0..64 {
        let mut piece_value = 0.0;
        let piece = board.get_square(i);
        let color = piece::get_color(piece);

        match color {
            piece::WHITE => {
                piece_value += piece_values.get(&piece::get_piece(piece)).unwrap_or(&0.0);
            }
            piece::BLACK => {
                piece_value -= piece_values.get(&piece::get_piece(piece)).unwrap_or(&0.0);
            }
            _ => {}
        }

        let position_value = 0.5 - (i.abs_diff(31) as f32/ 31.0) * 0.5;
        if color == piece::WHITE {
            piece_value += position_value;
        } else {
            piece_value -= position_value;
        }

        if piece::get_piece(piece) == piece::KING {
            if color == piece::WHITE {
                piece_value -= position_value;

                if i == 62 || i == 58 {
                    piece_value += 0.75;
                }
            } else {
                piece_value += position_value;

                if i == 6 || i == 2 {
                    piece_value -= 0.75;
                }
            }
        }

        score += piece_value;
    }

    if board.current_color_turn == piece::WHITE {
        score
    } else {
        -score
    }
}

fn search(board: &Board, depth: u32, mut alpha: f32, beta: f32) -> f32 {
    if depth == 0 {
        return quiescence_search(board, 5, alpha, beta);
    }

    let mut best_score = -f32::INFINITY;
    let legal_moves = board.get_legal_moves().unwrap_or_default();
    
    if legal_moves.is_empty() {
        if board.is_in_check(None) {
            return best_score;
        } else {
            return 0.0;
        }
    }

    for m in legal_moves {
        let mut new_board = *board;
        new_board.make_move(m);
        let score = -search(&new_board, depth - 1, -beta, -alpha);
        best_score = best_score.max(score);
        alpha = alpha.max(best_score);
        if alpha >= beta {
            break;
        }
        if std::time::Instant::now() > board.current_move_search_deadline {
            break;
        }
    }

    best_score
}

fn quiescence_search(board: &Board, max_depth: u32, mut alpha: f32, beta: f32) -> f32 {
    if max_depth == 0 {
        return evaluate(board);
    }

    let legal_moves = board.get_legal_moves().unwrap_or_default();
    let is_in_check = board.is_in_check(None);
    if legal_moves.is_empty() {
        return if is_in_check { -f32::INFINITY } else { 0.0 };
    }

    if !is_in_check {
        let baseline = evaluate(board);
        if baseline >= beta {
            return baseline;
        }
        alpha = alpha.max(baseline);
    }

    for m in legal_moves {
        if !is_in_check && !m.is_capture() {
            continue;
        }
        let mut new_board = *board;
        new_board.make_move(m);
        let score = -quiescence_search(&new_board, max_depth - 1, -beta, -alpha);
        if score >= beta {
            return score;;
        }
        alpha = alpha.max(score);
        if std::time::Instant::now() > board.current_move_search_deadline {
            break;
        }
    }

    alpha
}

pub fn find_best_move(board: &mut Board, depth: u32, max_time: std::time::Duration) -> Option<Move> {
    let mut best_score = -f32::INFINITY;
    let mut best_move = None;
    let mut alpha = -f32::INFINITY;
    let beta = f32::INFINITY;

    board.current_move_search_deadline = std::time::Instant::now() + max_time;
    for m in board.get_legal_moves()? {
        if best_move.is_none() {
            best_move = Some(m);
        }
        let mut new_board = *board;
        new_board.make_move(m);
        let score = -search(&new_board, depth - 1, -beta, -alpha);
        if score > best_score || best_move.is_none() {
            best_score = score;
            best_move = Some(m);
        }

        alpha = alpha.max(best_score);
        if alpha >= beta {
            break;
        }
        if std::time::Instant::now() > board.current_move_search_deadline {
            break;
        }
    }

    best_move
}
    
pub fn play(board: &mut Board) {
    let best_move = find_best_move(board, 4, std::time::Duration::from_secs(5));
    match best_move {
        Some(m) => {
            board.make_move(m);
        }
        None => {
            return;
        }
    }
}