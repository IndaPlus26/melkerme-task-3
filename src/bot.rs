use std::collections::HashMap;

use crate::board::Board;
use crate::r#move::Move;
use crate::piece;

use std::time::{Duration, Instant};

type MoveTable = HashMap<u64, Move>;

#[derive(Clone, Copy)]
enum Bound {
    Exact,
    Lower,
    Upper,
}

struct TableEntry {
    depth: u32,
    score: f32,
    bound: Bound,
    best_move: Move,
}

type SearchTable = HashMap<u64, TableEntry>;

fn check_time(deadline: Instant) -> Result<(), u8> {
    if Instant::now() > deadline {
        return Err(1);
    }
    Ok(())
}

const MAX_SEARCH_PLY: usize = 128;
const MATE_SCORE: f32 = 10_000.0;
const MATE_THRESHOLD: f32 = MATE_SCORE - MAX_SEARCH_PLY as f32;

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

fn ordering_value(p: u8) -> i32 {
    match piece::get_piece(p) {
        piece::PAWN => 100,
        piece::KNIGHT => 300,
        piece::BISHOP => 310,
        piece::ROOK => 500,
        piece::QUEEN => 900,
        piece::KING => 10_000,
        _ => 0,
    }
}

fn move_priority(board: &Board, m: &Move) -> i32 {
    let mut priority = 0;

    if m.is_capture() {
        let victim = if m.is_en_passant() {
            100
        } else {
            ordering_value(board.get_square(m.to))
        };

        let attacker = ordering_value(board.get_square(m.from));

        // Prefer valuable victims and cheaper attackers.
        priority += 10_000 + 10 * victim - attacker;
    }

    if m.is_promotion() {
        priority += 20_000 + ordering_value(m.get_promotion_piece());
    }

    priority
}

fn order_moves(board: &Board, moves: &mut [Move]) {
    moves.sort_by_cached_key(|m| std::cmp::Reverse(move_priority(board, m)));
}

fn evaluate(board: &Board) -> f32 {
    let mut score = 0.0;

    // Piece values
    for i in 0..64 {
        let mut piece_value = 0.0;
        let piece = board.get_square(i);
        let color = piece::get_color(piece);

        let material = match piece::get_piece(piece) {
            piece::PAWN => 1.0,
            piece::KNIGHT => 3.0,
            piece::BISHOP => 3.1,
            piece::ROOK => 5.0,
            piece::QUEEN => 9.0,
            _ => 0.0,
        };

        match color {
            piece::WHITE => {
                piece_value += material;
            }
            piece::BLACK => {
                piece_value -= material;
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
    /* score += (board
    .get_legal_moves(Some(piece::WHITE))
    .unwrap_or_default()
    .len() as f32
    - board
        .get_legal_moves(Some(piece::BLACK))
        .unwrap_or_default()
        .len() as f32)
    * 0.03; */

    if board.current_color_turn == piece::WHITE {
        score
    } else {
        -score
    }
}

fn score_to_table(score: f32, ply: usize) -> f32 {
    if score >= MATE_THRESHOLD {
        score + ply as f32
    } else if score <= -MATE_THRESHOLD {
        score - ply as f32
    } else {
        score
    }
}

fn score_from_table(score: f32, ply: usize) -> f32 {
    if score >= MATE_THRESHOLD {
        score - ply as f32
    } else if score <= -MATE_THRESHOLD {
        score + ply as f32
    } else {
        score
    }
}

#[derive(Default)]
struct SearchStats {
    nodes: u64,
    qnodes: u64,
    tt_cutoffs: u64,
    tt_hits: u64,
    tt_usable: u64,
    tt_depth_ok: u64,
}

fn search(
    board: &Board,
    depth: u32,
    ply: usize,
    history: &mut [u64; MAX_SEARCH_PLY],
    move_table: &mut SearchTable,
    qmove_table: &mut MoveTable,
    mut alpha: f32,
    beta: f32,
    deadline: Instant,
    stats: &mut SearchStats,
) -> Result<f32, u8> {
    check_time(deadline)?;
    if ply >= MAX_SEARCH_PLY {
        return Err(2);
    }
    if depth == 0 {
        return quiescence_search(
            board,
            ply,
            history,
            qmove_table,
            alpha,
            beta,
            deadline,
            stats,
        );
    }

    stats.nodes += 1;

    let key = board.position_key();
    if history[..ply].contains(&key) {
        return Ok(0.0);
    }
    history[ply] = key;

    let original_alpha = alpha;

    if let Some(entry) = move_table.get(&key) {
        stats.tt_hits += 1;
        if entry.depth >= depth {
            stats.tt_depth_ok += 1;
        }
        if entry.depth >= depth {
            stats.tt_usable += 1;
            let score = score_from_table(entry.score, ply);

            let usable = match entry.bound {
                Bound::Exact => true,
                Bound::Lower => score >= beta,
                Bound::Upper => score <= alpha,
            };

            if usable {
                stats.tt_cutoffs += 1;
                return Ok(score);
            }
        }
    }

    let mut best_score = -f32::INFINITY;
    let mut legal_moves = board.get_legal_moves(None).unwrap_or_default();
    order_moves(board, &mut legal_moves);

    if let Some(entry) = move_table.get(&key) {
        if let Some(index) = legal_moves.iter().position(|m| *m == entry.best_move) {
            legal_moves[..=index].rotate_right(1);
        }
    }

    if legal_moves.is_empty() {
        if board.is_in_check(None) {
            return Ok(-MATE_SCORE + ply as f32);
        } else {
            return Ok(0.0);
        }
    }

    let mut best_move = None;
    for m in legal_moves {
        check_time(deadline)?;
        let mut new_board = *board;
        new_board.make_move(m);
        let score = -search(
            &new_board,
            depth - 1,
            ply + 1,
            history,
            move_table,
            qmove_table,
            -beta,
            -alpha,
            deadline,
            stats,
        )?;
        if best_move.is_none() || score > best_score {
            best_score = score;
            best_move = Some(m);
        }
        alpha = alpha.max(best_score);
        if alpha >= beta {
            break;
        }
    }

    if let Some(m) = best_move {
        let bound = if best_score <= original_alpha {
            Bound::Upper
        } else if best_score >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };

        move_table.insert(
            key,
            TableEntry {
                depth,
                score: score_to_table(best_score, ply),
                bound,
                best_move: m,
            },
        );
    }

    Ok(best_score)
}

fn quiescence_search(
    board: &Board,
    ply: usize,
    history: &mut [u64; MAX_SEARCH_PLY],
    qmove_table: &mut MoveTable,
    mut alpha: f32,
    beta: f32,
    deadline: Instant,
    stats: &mut SearchStats,
) -> Result<f32, u8> {
    check_time(deadline)?;
    if ply >= MAX_SEARCH_PLY {
        return Err(2);
    }
    stats.qnodes += 1;

    let key = board.position_key();
    if history[..ply].contains(&key) {
        return Ok(0.0);
    }
    history[ply] = key;

    let is_in_check = board.is_in_check(None);
    let mut legal_moves = if is_in_check {
        board.get_legal_moves(None).unwrap_or_default()
    } else {
        board.get_legal_tactical_moves()
    };

    if legal_moves.is_empty() {
        if is_in_check {
            return Ok(-MATE_SCORE + ply as f32);
        }
        if !board.has_legal_moves() {
            return Ok(0.0);
        }
    }

    if !is_in_check {
        let baseline = evaluate(board);
        if baseline >= beta {
            return Ok(baseline);
        }
        alpha = alpha.max(baseline);
    }

    order_moves(board, &mut legal_moves);

    if let Some(&preferred) = qmove_table.get(&key) {
        if let Some(index) = legal_moves.iter().position(|m| *m == preferred) {
            legal_moves[..=index].rotate_right(1);
        }
    }

    let mut improving_move = None;

    for m in legal_moves {
        check_time(deadline)?;

        let mut new_board = *board;
        new_board.make_move(m);

        let score = -quiescence_search(
            &new_board,
            ply + 1,
            history,
            qmove_table,
            -beta,
            -alpha,
            deadline,
            stats,
        )?;

        if score >= beta {
            // This move proved good enough to cause a cutoff.
            qmove_table.insert(key, m);
            return Ok(score);
        }

        if score > alpha {
            alpha = score;
            improving_move = Some(m);
        }
    }

    if let Some(m) = improving_move {
        qmove_table.insert(key, m);
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

    let mut move_table = SearchTable::new();
    let mut qmove_table = MoveTable::new();

    for d in 1..=depth {
        let started_at = Instant::now();
        let mut stats = SearchStats::default();

        let mut history = [0u64; MAX_SEARCH_PLY];
        history[0] = board.position_key();

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
            let score = match search(
                &new_board,
                d - 1,
                1,
                &mut history,
                &mut move_table,
                &mut qmove_table,
                -beta,
                -alpha,
                deadline,
                &mut stats,
            ) {
                Ok(score) => -score,
                Err(reason) => {
                    let message = match reason {
                        1 => "Timeout",
                        2 => "Search-ply limit reached",
                        _ => "Search interrupted",
                    };

                    println!("{} at depth {}", message, d);
                    return best_move;
                }
            };
            if score > best_score || best_move.is_none() {
                best_score = score;
                best_index = index;
                if best_score == MATE_SCORE - 1.0 {
                    return best_move;
                }
            }
            alpha = alpha.max(best_score);
            if alpha >= beta {
                break;
            }
        }
        best_move = Some(moves[best_index]);
        moves.swap(0, best_index);

        let seconds = started_at.elapsed().as_secs_f64();
        let total = stats.nodes + stats.qnodes;

        println!(
            "Depth {} complete | score {:.3} | {:.2}s | nodes {} | qnodes {} | TT cutoffs {} | TT hits {} | TT depth ok {} | TT usable {} | {:.0} nodes/s",
            d,
            best_score,
            seconds,
            stats.nodes,
            stats.qnodes,
            stats.tt_cutoffs,
            stats.tt_hits,
            stats.tt_depth_ok,
            stats.tt_usable,
            total as f64 / seconds.max(0.000_001),
        );

        match check_time(deadline) {
            Ok(_) => {}
            Err(reason) => {
                let message = match reason {
                    1 => "Timeout",
                    2 => "Search-ply limit reached",
                    _ => "Search interrupted",
                };

                println!("{} at depth {}", message, d);
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
