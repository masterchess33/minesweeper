use crate::board::Cell;

pub enum PlayResult {
    GameOver,
    Safe,
    Invalid
}

pub fn play(x: usize, y: usize, board: &mut Vec<Vec<Cell>>) -> PlayResult {
    if x < board.len() && y < board.len(){
        board[x][y].discovered = true;
        return if board[x][y].mine {
            PlayResult::GameOver
        }else
        {
            PlayResult::Safe
        }
    }
    return PlayResult::Invalid;
}

pub fn create_game(){

}
