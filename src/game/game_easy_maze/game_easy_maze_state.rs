use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Copy, Clone, Hash)]
pub struct GameEasyMazeState {
    row: i32,
    column: i32,
}

impl GameEasyMazeState {
    pub const fn new(row: i32, column: i32) -> GameEasyMazeState {
        GameEasyMazeState {
            row,
            column,
        }
    }

    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_column(&self) -> i32 {
        self.column
    }

    pub fn step(&mut self, diff_row: i32, diff_column: i32) {
        self.row += diff_row;
        self.column += diff_column;
    }

}

impl PartialEq for GameEasyMazeState {
    fn eq(&self, other: &GameEasyMazeState) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Eq for GameEasyMazeState {}