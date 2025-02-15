use std::collections::{HashMap, HashSet, VecDeque};

use crate::{Board, Move};
use boomphf::Mphf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Strategy {
    hash: Mphf<Board>,
    vals: Vec<u8>,
}

impl Strategy {
    pub fn new(boards: &Vec<Board>, vals: &Vec<f64>, gamma: f64) -> Self {
        assert!(boards.len() == vals.len());
        let hash = Mphf::new(gamma, boards);
        let mut v = Vec::new();
        v.resize(vals.len(), 0);
        for i in 0..boards.len() {
            v[hash.hash(&boards[i]) as usize] = (vals[i] * (u8::MAX as f64)) as u8;
        }
        Strategy { hash, vals: v }
    }

    pub fn win_rate(&self, b: &Board) -> f32 {
        let quantized_val = self.vals[self.hash.hash(&b.canonical_orientation()) as usize];
        (quantized_val as f32) / (u8::MAX as f32)
    }

    pub fn best_move(&self, b: &Board) -> Move {
        let mut best_win_rate = -1.0;
        let mut best_move = Move::Left;

        for (b, m) in b.possible_moves() {
            let (num, denom) = b
                .possible_new_tiles()
                .map(|b| self.win_rate(&b))
                .fold((0.0, 0.0), |(arate, acount), rate| {
                    (arate + rate, acount + 1.0)
                });
            let win_rate = num / denom;
            if win_rate > best_win_rate {
                best_win_rate = win_rate;
                best_move = m;
            }
        }

        best_move
    }
}

pub fn optimal_strategy() -> Strategy {
    let mut boards: HashSet<Board> = HashSet::new();
    let mut eval_order: Vec<Board> = vec![]; // Order to evaluate position optimal win rates
    let mut queue = VecDeque::new();

    // Force calculating the value of the starting position
    eval_order.push(Board::empty());

    // Starting boards have two random tiles
    for b in Board::empty().possible_new_tiles() {
        for b2 in b.possible_new_tiles() {
            if b2 == b2.canonical_orientation() && !queue.contains(&b2) {
                queue.push_front(b2);

                eval_order.push(b2);
            }
        }
    }

    // Search in breadth-first order so we can traverse `eval_order` in reverse order
    // while always knowing the child positions will have known win rates
    while let Some(b) = queue.pop_back() {
        if boards.contains(&b) {
            continue;
        }
        boards.insert(b);
        for (b2, _) in b.possible_moves() {
            for b2 in b2.possible_new_tiles() {
                let b2 = b2.canonical_orientation();
                queue.push_front(b2);
                eval_order.push(b2);
            }
        }
    }

    let mut win_rate: HashMap<Board, f64> = HashMap::new();
    for board in eval_order.iter().rev() {
        assert!(*board == board.canonical_orientation());
        if !win_rate.contains_key(board) {
            let mut best_win_rate = -1.0;
            for (b, _) in board.possible_moves() {
                let mut possible_tiles: f64 = 0.0;
                let mut win_rate_sum: f64 = 0.0;
                for b in b.possible_new_tiles() {
                    win_rate_sum += win_rate
                        .get(&b.canonical_orientation())
                        .expect("position_win_rates contain all child positions");
                    possible_tiles += 1.0;
                }
                let win_rate = win_rate_sum / possible_tiles;
                if win_rate > best_win_rate {
                    best_win_rate = win_rate;
                }
                assert!(possible_tiles >= 1.0);
            }
            if best_win_rate == -1.0 {
                if board.is_win() {
                    win_rate.insert(*board, 1.0);
                } else if board.is_loss() {
                    win_rate.insert(*board, 0.0);
                } else {
                    unreachable!();
                }
            } else {
                win_rate.insert(*board, best_win_rate);
                assert!(best_win_rate <= 1.0);
                assert!(best_win_rate >= 0.0);
            }
        }
    }

    let win_rates: Vec<(Board, f64)> = win_rate.into_iter().collect();

    let keys: Vec<_> = win_rates.iter().map(|(b, _)| *b).collect();
    let vals: Vec<_> = win_rates.iter().map(|(_, s)| *s).collect();

    Strategy::new(&keys, &vals, 2.0)
}
