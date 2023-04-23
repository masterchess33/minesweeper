use rand::Rng;


pub struct Cell {
    pub mine: bool,
    pub discovered: bool,
    pub number: u8
}

pub enum Size {
    Small,
    Medium,
    Big
}

pub enum Density{
    Light,
    Dense
}

impl Default for Cell{
    fn default() -> Cell {
        Cell{
            mine: false,
            discovered: true,
            number:0
        }
    }
}



pub fn create_board(size: Size) -> Vec<Vec<Cell>>{
    let mut board: Vec<Vec<Cell>> = vec![];
    let length = match size {
        Size::Small => 8,
        Size::Medium => 16,
        Size::Big => 32
    };
    for i in 0..length {
        board.push(vec![]);
        for _j in 0..length {
            board[i].push(Cell::default())
        }
    }
    return board;
}

pub fn instantiate_board(board: &mut Vec<Vec<Cell>>){
    let size = board.len();
    for i in 0..size {
        for j in 0..size {
            board[i][j].discovered = true;
            match set_mine(Density::Light) {
                true => {
                    board[i][j].mine = true;
                    try_increase_cell_number(i.checked_sub(1),Some(j), board);
                    try_increase_cell_number(i.checked_sub(1), j.checked_sub(1), board);
                    try_increase_cell_number(i.checked_sub(1), j.checked_add(1), board);
                    try_increase_cell_number(Some(i), j.checked_sub(1), board);
                    try_increase_cell_number(Some(i), j.checked_add(1), board);
                    try_increase_cell_number(i.checked_add(1), Some(j), board);
                    try_increase_cell_number(i.checked_add(1), j.checked_sub(1), board);
                    try_increase_cell_number(i.checked_add(1), j.checked_add(1), board);
                }

                false => {}
            }
        }
    }
}

fn set_mine(density: Density) -> bool{
    let mut rng = rand::thread_rng();

    let  sample = match density {
        Density::Light => rng.gen_range(0..15),
        Density::Dense => rng.gen_range(0..10)
    };

    if (0..2).contains(&sample) {
        true
    }else {
        false
    }
}

fn try_increase_cell_number(x: Option<usize>, y: Option<usize>, board: &mut Vec<Vec<Cell>>) -> (){
    if x == None || y == None {
        return;
    }else {
        if x.unwrap() < board.len() && y.unwrap() < board.len() {
            board[x.unwrap()][y.unwrap()].number += 1;
            return;
        }else {
            return;
        }
    }

}

