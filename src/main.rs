use crate::board::Board;

mod board;

fn main() {
    let board = Board::new();
    println!("Debug: {:?}", board);
}
