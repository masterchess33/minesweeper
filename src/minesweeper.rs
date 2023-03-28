use std::os::raw::c_char;
use rand::Rng;


pub struct Cell {
    mine: bool,
    discovered: bool,
    number: u8
}

enum Size {
    Small,
    Medium,
    Big
}

impl Default for Cell{
    fn default() -> Cell {
        Cell{
            mine: false,
            discovered: false,
            number:0
        }
    }
}


pub fn create_board(size: Size) -> Vec<Vec<Cell>>{
    let mut board: Vec<Vec<Cell>> = vec![];
    let h = match size {
        Size::Small => 8,
        Size::Medium => 16,
        Size::Big => 32
    };
    for i in 0..h {
        board.push(vec![]);
        for j in 0..h {
            board[i].push(Cell::default())
        }
    }
    return board;
}

pub fn set_mines(density: u8) -> bool{
    let mut rng = rand::thread_rng();

    let  sample: u8 = match density {
        0..=10 => rng.gen_range(0..100),
        11..=20 => rng.gen_range(0..60),
        _ => {}
    };
    return match sample {
        0..=20 => true,
        21.. => false,
        _ => false
    }
}