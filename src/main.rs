use crate::minesweeper::{instantiate_board, Size};

mod minesweeper;
mod terminal_view;

fn main() {

    let mut board = minesweeper::create_board(Size::Small);
    instantiate_board(&mut board);
    terminal_view::print_board(board);
}
