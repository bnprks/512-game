mod board;
mod strategy;
mod wasm;

pub use board::Board;
pub use board::Move;

pub use strategy::optimal_strategy;
pub use strategy::Strategy;

pub use wasm::save_strategy;
pub use wasm::WasmStrategy;
