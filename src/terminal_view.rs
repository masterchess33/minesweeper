use crate::board::Cell;

pub fn print_board( board: Vec<Vec<Cell>>){
    for i in board.into_iter().rev() {
        for j in i.into_iter().rev() {
            if j.discovered == true {
                if j.number ==0 {
                    print!("[ ]");
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