mod shared;

// pub use shared::play;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn play(perfect_player: Vec<u32>, opponent: Vec<u32>, first_hand: bool) -> i32 {
    shared::play(perfect_player, opponent, first_hand)
}
