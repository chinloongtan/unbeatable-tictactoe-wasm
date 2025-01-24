// for debug use because lib.rs cannot build with `cargo run`
mod shared;
use shared::play;

fn main() {
    println!("Start");
    let step = play(vec![], vec![], true);
    println!("Next step: {}", step);
}
