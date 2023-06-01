use crate::board::{instantiate_board, Size};

mod board;
mod terminal_view;
mod game;

fn main() {

    let mut board = board::create_board(Size::Small);
    instantiate_board(&mut board);
    terminal_view::print_board(board);
}
