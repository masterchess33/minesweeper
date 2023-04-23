use crate::minesweeper::Cell;

pub fn print_board( board: Vec<Vec<Cell>>){
    for i in board.into_iter().rev() {
        for j in i.into_iter().rev() {
            if j.discovered == true {
                if j.mine {
                    print!("[*]");
                }else {
                    print!("[{}]",j.number);
                }
            }else {
                print!("[\u{2591}]");
            }
        }
        println!();
    }
}