use std::collections::HashMap;

use crate::board::Board;
use crate::r#move::Move;
use crate::piece;

use std::time::{Duration, Instant};

fn check_time(deadline: Instant) -> Result<(), u8> {
    if Instant::now() > deadline {
        return Err(1);
    }
    Ok(())
}

pub const PAWN_PST: [f32; 64] = [
    0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.60, 0.60, 0.65, 0.70, 0.70, 0.65, 0.60, 0.60,
    0.25, 0.28, 0.35, 0.45, 0.45, 0.35, 0.28, 0.25, 0.12, 0.15, 0.22, 0.35, 0.35, 0.22, 0.15, 0.12,
    0.05, 0.08, 0.15, 0.28, 0.28, 0.15, 0.08, 0.05, 0.02, 0.03, 0.08, 0.15, 0.15, 0.08, 0.03, 0.02,
    0.02, 0.04, 0.04, -0.10, -0.10, 0.04, 0.04, 0.02, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00,
    0.00,
];

pub const KNIGHT_PST: [f32; 64] = [
    -0.45, -0.30, -0.20, -0.18, -0.18, -0.20, -0.30, -0.45, -0.28, -0.10, 0.02, 0.06, 0.06, 0.02,
    -0.10, -0.28, -0.18, 0.05, 0.18, 0.24, 0.24, 0.18, 0.05, -0.18, -0.15, 0.10, 0.25, 0.32, 0.32,
    0.25, 0.10, -0.15, -0.15, 0.08, 0.22, 0.30, 0.30, 0.22, 0.08, -0.15, -0.18, 0.02, 0.15, 0.20,
    0.20, 0.15, 0.02, -0.18, -0.30, -0.12, -0.02, 0.02, 0.02, -0.02, -0.12, -0.30, -0.45, -0.25,
    -0.20, -0.18, -0.18, -0.20, -0.25, -0.45,
];

pub const BISHOP_PST: [f32; 64] = [
    -0.18, -0.10, -0.08, -0.08, -0.08, -0.08, -0.10, -0.18, -0.08, 0.04, 0.05, 0.06, 0.06, 0.05,
    0.04, -0.08, -0.05, 0.08, 0.12, 0.16, 0.16, 0.12, 0.08, -0.05, -0.05, 0.10, 0.15, 0.20, 0.20,
    0.15, 0.10, -0.05, -0.05, 0.08, 0.16, 0.20, 0.20, 0.16, 0.08, -0.05, -0.05, 0.10, 0.12, 0.15,
    0.15, 0.12, 0.10, -0.05, -0.08, 0.12, 0.05, 0.05, 0.05, 0.05, 0.12, -0.08, -0.18, -0.08, -0.12,
    -0.08, -0.08, -0.12, -0.08, -0.18,
];

pub const ROOK_PST: [f32; 64] = [
    0.08, 0.10, 0.12, 0.15, 0.15, 0.12, 0.10, 0.08, 0.20, 0.24, 0.24, 0.26, 0.26, 0.24, 0.24, 0.20,
    -0.03, 0.00, 0.03, 0.05, 0.05, 0.03, 0.00, -0.03, -0.05, 0.00, 0.03, 0.05, 0.05, 0.03, 0.00,
    -0.05, -0.05, 0.00, 0.03, 0.05, 0.05, 0.03, 0.00, -0.05, -0.05, 0.00, 0.03, 0.05, 0.05, 0.03,
    0.00, -0.05, -0.05, 0.00, 0.02, 0.04, 0.04, 0.02, 0.00, -0.05, 0.00, 0.00, 0.05, 0.10, 0.10,
    0.05, 0.00, 0.00,
];

pub const QUEEN_PST: [f32; 64] = [
    -0.15, -0.10, -0.08, -0.05, -0.05, -0.08, -0.10, -0.15, -0.10, 0.00, 0.02, 0.03, 0.03, 0.02,
    0.00, -0.10, -0.08, 0.02, 0.06, 0.08, 0.08, 0.06, 0.02, -0.08, -0.05, 0.03, 0.08, 0.12, 0.12,
    0.08, 0.03, -0.05, -0.05, 0.03, 0.08, 0.12, 0.12, 0.08, 0.03, -0.05, -0.08, 0.02, 0.06, 0.08,
    0.08, 0.06, 0.02, -0.08, -0.10, 0.00, 0.02, 0.03, 0.03, 0.02, 0.00, -0.10, -0.15, -0.10, -0.08,
    0.00, 0.00, -0.08, -0.10, -0.15,
];

pub const KING_MG_PST: [f32; 64] = [
    -0.55, -0.65, -0.70, -0.80, -0.80, -0.70, -0.65, -0.55, -0.50, -0.60, -0.65, -0.75, -0.75,
    -0.65, -0.60, -0.50, -0.45, -0.55, -0.60, -0.70, -0.70, -0.60, -0.55, -0.45, -0.40, -0.50,
    -0.55, -0.65, -0.65, -0.55, -0.50, -0.40, -0.30, -0.40, -0.45, -0.55, -0.55, -0.45, -0.40,
    -0.30, -0.15, -0.25, -0.30, -0.40, -0.40, -0.30, -0.25, -0.15, 0.10, 0.10, -0.05, -0.20, -0.20,
    -0.05, 0.10, 0.10, 0.15, 0.25, 0.30, -0.10, -0.15, 0.00, 0.35, 0.20,
];

pub const KING_EG_PST: [f32; 64] = [
    -0.40, -0.25, -0.15, -0.10, -0.10, -0.15, -0.25, -0.40, -0.25, -0.10, 0.05, 0.10, 0.10, 0.05,
    -0.10, -0.25, -0.15, 0.05, 0.20, 0.30, 0.30, 0.20, 0.05, -0.15, -0.10, 0.10, 0.30, 0.40, 0.40,
    0.30, 0.10, -0.10, -0.10, 0.10, 0.30, 0.40, 0.40, 0.30, 0.10, -0.10, -0.15, 0.05, 0.20, 0.30,
    0.30, 0.20, 0.05, -0.15, -0.25, -0.10, 0.05, 0.10, 0.10, 0.05, -0.10, -0.25, -0.40, -0.25,
    -0.15, -0.10, -0.10, -0.15, -0.25, -0.40,
];

fn evaluate(board: &Board) -> f32 {
    let mut score = 0.0;

    let piece_values: HashMap<u8, f32> = HashMap::from([
        (piece::PAWN, 1.0),
        (piece::KNIGHT, 3.0),
        (piece::BISHOP, 3.1),
        (piece::ROOK, 5.0),
        (piece::QUEEN, 9.0),
    ]);

    // Piece values
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

        // Position score table
        let pst_index = if color == piece::WHITE { i } else { i ^ 56 };
        piece_value += match piece::get_piece(piece) {
            piece::PAWN => {
                if color == piece::WHITE {
                    PAWN_PST[pst_index]
                } else {
                    -PAWN_PST[pst_index]
                }
            }
            piece::KNIGHT => {
                if color == piece::WHITE {
                    KNIGHT_PST[pst_index]
                } else {
                    -KNIGHT_PST[pst_index]
                }
            }
            piece::BISHOP => {
                if color == piece::WHITE {
                    BISHOP_PST[pst_index]
                } else {
                    -BISHOP_PST[pst_index]
                }
            }
            piece::ROOK => {
                if color == piece::WHITE {
                    ROOK_PST[pst_index]
                } else {
                    -ROOK_PST[pst_index]
                }
            }
            piece::QUEEN => {
                if color == piece::WHITE {
                    QUEEN_PST[pst_index]
                } else {
                    -QUEEN_PST[pst_index]
                }
            }
            piece::KING => {
                if color == piece::WHITE {
                    KING_MG_PST[pst_index]
                } else {
                    -KING_MG_PST[pst_index]
                }
            }
            _ => 0.0,
        };

        score += piece_value;
    }

    // Mobility score
    score += (board
        .get_legal_moves(Some(piece::WHITE))
        .unwrap_or_default()
        .len() as f32
        - board
            .get_legal_moves(Some(piece::BLACK))
            .unwrap_or_default()
            .len() as f32)
        * 0.03;

    if board.current_color_turn == piece::WHITE {
        score
    } else {
        -score
    }
}

fn search(
    board: &Board,
    depth: u32,
    mut alpha: f32,
    beta: f32,
    deadline: Instant,
) -> Result<f32, u8> {
    check_time(deadline)?;
    if depth == 0 {
        return quiescence_search(board, 5, alpha, beta, deadline);
    }

    let mut best_score = -f32::INFINITY;
    let legal_moves = board.get_legal_moves(None).unwrap_or_default();

    if legal_moves.is_empty() {
        if board.is_in_check(None) {
            return Ok(best_score);
        } else {
            return Ok(0.0);
        }
    }

    for m in legal_moves {
        check_time(deadline)?;
        let mut new_board = *board;
        new_board.make_move(m);
        let score = -search(&new_board, depth - 1, -beta, -alpha, deadline)?;
        best_score = best_score.max(score);
        alpha = alpha.max(best_score);
        if alpha >= beta {
            break;
        }
    }

    Ok(best_score)
}

fn quiescence_search(
    board: &Board,
    max_depth: u32,
    mut alpha: f32,
    beta: f32,
    deadline: Instant,
) -> Result<f32, u8> {
    check_time(deadline)?;

    if max_depth == 0 {
        return Ok(evaluate(board));
    }

    let legal_moves = board.get_legal_moves(None).unwrap_or_default();
    let is_in_check = board.is_in_check(None);
    if legal_moves.is_empty() {
        return if is_in_check {
            Ok(-f32::INFINITY)
        } else {
            Ok(0.0)
        };
    }

    if !is_in_check {
        let baseline = evaluate(board);
        if baseline >= beta {
            return Ok(baseline);
        }
        alpha = alpha.max(baseline);
    }

    for m in legal_moves {
        check_time(deadline)?;
        if !is_in_check && !m.is_capture() {
            continue;
        }
        let mut new_board = *board;
        new_board.make_move(m);
        let score = -quiescence_search(&new_board, max_depth - 1, -beta, -alpha, deadline)?;
        if score >= beta {
            return Ok(score);
        }
        alpha = alpha.max(score);
    }

    Ok(alpha)
}

pub fn find_best_move(board: &mut Board, depth: u32, max_time: Duration) -> Option<Move> {
    let mut moves = board.get_legal_moves(None).unwrap_or_default();
    if moves.is_empty() {
        return None;
    }
    let mut best_move = moves.first().copied();

    let deadline = Instant::now() + max_time;

    for d in 1..=depth {
        let mut alpha = -f32::INFINITY;
        let beta = f32::INFINITY;
        let mut best_score = -f32::INFINITY;
        let mut best_index = 0;

        for (index, &m) in moves.iter().enumerate() {
            if best_move.is_none() {
                best_move = Some(m);
            }
            let mut new_board = *board;
            new_board.make_move(m);
            let score = match search(&new_board, d - 1, -beta, -alpha, deadline) {
                Ok(score) => -score,
                Err(_) => {
                    return best_move;
                }
            };
            if score > best_score || best_move.is_none() {
                best_score = score;
                best_index = index;
            }
            alpha = alpha.max(best_score);
            if alpha >= beta {
                break;
            }
        }
        best_move = Some(moves[best_index]);
        moves.swap(0, best_index);
        match check_time(deadline) {
            Ok(_) => {}
            Err(_) => {
                return best_move;
            }
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
