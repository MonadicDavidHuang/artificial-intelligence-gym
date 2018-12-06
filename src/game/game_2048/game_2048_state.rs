use std::fmt::Display;
use std::fmt::{Formatter, Result};

const BOARD_SIZE: usize = 4;
const EMPTY_LITERAL: i32 = 0;

#[derive(Debug, Copy, Clone)]
pub struct Game2048State {
    board: [[i32; BOARD_SIZE]; BOARD_SIZE],
}

impl Game2048State {
    pub const fn new() -> Game2048State {
        let state = Game2048State {
            board: [[EMPTY_LITERAL; BOARD_SIZE]; BOARD_SIZE],
        };

        state
    }

    pub const fn get_board_size() -> usize {
        BOARD_SIZE
    }

    pub const fn get_empty_literal() -> i32 {
        EMPTY_LITERAL
    }

    pub fn set_element(&mut self, i: usize, j: usize, elem: i32) {
        self.board[i][j] = elem;
    }

    pub fn get_element(&self, i: usize, j: usize) -> i32 {
        self.board[i][j]
    }

    pub fn double(&mut self, i: usize, j: usize) {
        self.board[i][j] *= 2;
    }

    pub fn slide(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        self.board[i1][j1] = self.board[i2][j2];
        self.board[i2][j2] = EMPTY_LITERAL;
    }

    pub fn is_same(&self, i1: usize, j1: usize, i2: usize, j2: usize) -> bool {
        self.board[i1][j1] == self.board[i2][j2]
    }

    pub fn is_empty(&self, i: usize, j: usize) -> bool {
        self.board[i][j] == EMPTY_LITERAL
    }

    pub fn has_empty(&self) -> bool {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.is_empty(i, j) {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_board_ref(&self) -> &[[i32; BOARD_SIZE]; BOARD_SIZE] {
        &(self.board)
    }
}

impl PartialEq for Game2048State {
    fn eq(&self, other: &Game2048State) -> bool {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if other.board[i][j] != self.board[i][j] {
                    return false;
                }
            }
        }

        true
    }
}

impl Eq for Game2048State {}

// TODO: fix to use formatter
impl Display for Game2048State {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let board_size = Game2048State::get_board_size();

        print!("---");
        for _i in 0..board_size {
            print!("---------");
        }
        println!();

        print!("   ");
        for i in 0..board_size {
            print!("[{:7}]", i);
        }
        println!();

        for i in 0..board_size {
            print!("[{}]", i);
            for j in 0..board_size {
                if self.is_empty(i, j) {
                    print!("{:8} ", -1);
                } else {
                    print!("{:8} ", self.get_element(i, j));
                }
            }
            println!();
        }

        print!("---");
        for _i in 0..board_size {
            print!("---------");
        }
        println!();

        write!(f, "{}", "")
    }
}

#[cfg(test)]
mod tests_game_2048_state {
    use game::game_2048::game_2048_state::Game2048State;
    use game::game_2048::game_2048_state::BOARD_SIZE;

    #[test]
    fn test_set_and_get() {
        let mut state = Game2048State::new();

        state.set_element(0, 0, 2);

        assert_eq!(state.get_element(0, 0), 2);
    }

    #[test]
    fn test_double() {
        let mut state = Game2048State::new();

        state.set_element(0, 0, 2);
        state.double(0, 0);

        assert_eq!(state.get_element(0, 0), 4);
    }

    #[test]
    fn test_slide() {
        let mut state = Game2048State::new();

        state.set_element(0, 0, 2);

        state.slide(1, 1, 0, 0);

        assert_eq!(state.get_element(1, 1), 2);
        assert_eq!(state.get_element(0, 0), 0);
    }

    #[test]
    fn test_is_same() {
        let mut state = Game2048State::new();

        assert_eq!(state.get_element(0, 0), state.get_element(1, 1));

        state.set_element(0, 0, 2);
        state.set_element(1, 1, 2);

        assert_eq!(state.get_element(0, 0), state.get_element(1, 1));
    }

    #[test]
    fn test_is_empty() {
        let state = Game2048State::new();

        assert_eq!(state.get_element(0, 0), Game2048State::get_empty_literal());
    }

    #[test]
    fn test_get_board_ref() {
        let state = Game2048State::new();

        let board_ref = state.get_board_ref();

        assert_eq!(board_ref[0][0], Game2048State::get_empty_literal());
    }

    #[test]
    fn test_copy() {
        let mut state = Game2048State::new();

        state.set_element(0, 0, 2);
        let copied_state = state;

        assert!(!state.is_empty(0, 0)); // assert not moved

        assert!(!copied_state.is_empty(0, 0)); // assert exactly copied
    }

    #[test]
    fn test_eq() {
        let state1 = Game2048State::new();
        let mut state2 = Game2048State::new();

        state2.set_element(0, 0, 4);

        assert_ne!(state1, state2);
    }

    #[test]
    fn test_has_empty() {
        let mut state = Game2048State::new();

        assert!(state.has_empty());

        fill_all(&mut state, 2);

        assert!(!state.has_empty());
    }

    fn fill_all(state: &mut Game2048State, element: i32) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                state.set_element(i, j, element);
            }
        }
    }
}
