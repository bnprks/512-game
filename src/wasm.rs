use crate::{Board, Move, Strategy};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmStrategy(Strategy);

#[wasm_bindgen]
impl WasmStrategy {
    #[wasm_bindgen(constructor)]
    pub fn load_strategy(bytes: &[u8]) -> WasmStrategy {
        WasmStrategy {
            0: bincode::deserialize(bytes).expect("bytes should hold bincode-encoded Strategy"),
        }
    }

    pub fn message(&self, board: &[u8]) -> String {
        let b = Board::create(board.try_into().expect("Array length 9"));
        if b.is_win() {
            return "Win!".into();
        }
        if b.is_loss() {
            return "Loss".into();
        }
        let mut scores = Vec::new();
        for m in [Move::Left, Move::Right, Move::Up, Move::Down] {
            let win_rate = if let Some(b) = b.do_move(m) {
                let (num, denom) = b
                    .possible_new_tiles()
                    .map(|b| self.0.win_rate(&b))
                    .fold((0.0, 0.0), |(arate, acount), rate| {
                        (arate + rate, acount + 1.0)
                    });
                format!("{:>3}%", (100.0 * num / denom) as i8)
            } else {
                "    ".into()
            };
            scores.push(win_rate);
        }
        format!(
            "L: {}, R: {}, U: {}, D: {}",
            scores[0], scores[1], scores[2], scores[3]
        )
    }
}

pub fn save_strategy<W>(strategy: &Strategy, writer: W)
where
    W: std::io::Write,
{
    bincode::serialize_into(writer, strategy).unwrap()
}
