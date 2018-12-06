use std::collections::HashMap;
use std::collections::HashSet;
use std::prelude::v1::Vec;

use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction;
use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction::Down;
use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction::Left;
use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction::Right;
use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction::Up;
use game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral;
use game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral::BLOCK;
use game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral::MOVABLE;
use game::game_easy_maze::game_easy_maze_state::GameEasyMazeState;

#[derive(Debug)]
pub struct GameEazyMazeEnvironment {
    grid: Vec<Vec<GameEasyMazeLiteral>>,
    state: GameEasyMazeState,
    default_reward: f64,
    move_prob: f64,
    row_length: usize,
    column_length: usize,
    actions: [GameEazyMazeAction; 4],
    movable_states: HashSet<GameEasyMazeState>,
}

// #[allow(dead_code)]
impl GameEazyMazeEnvironment {
    pub fn new(grid: Vec<Vec<GameEasyMazeLiteral>>, move_prob: f64) -> GameEazyMazeEnvironment {
        let state = GameEasyMazeState::new(-1, -1);

        let default_reward = -0.04;

        let row_length = (&grid).len();

        let column_length = (&grid[0]).len();

        let actions = [Up, Down, Left, Right];

        let movable_states = GameEazyMazeEnvironment::get_movable_states(grid.as_slice());

        GameEazyMazeEnvironment {
            grid,
            state,
            default_reward,
            move_prob,
            row_length,
            column_length,
            actions,
            movable_states,
        }
    }

    /// Get all move-able state, and only called once.
    fn get_movable_states(slice: &[Vec<GameEasyMazeLiteral>]) -> HashSet<GameEasyMazeState> {
        let mut movable_states = HashSet::new();

        let row_size = slice.len();
        let column_size = slice[0].len();

        for i in 0..row_size {
            for j in 0..column_size {
                if slice[i][j] == MOVABLE {
                    movable_states.insert(GameEasyMazeState::new(i as i32, j as i32));
                }
            }
        }

        movable_states
    }

    fn row_length(&self) -> usize {
        self.row_length
    }

    fn column_length(&self) -> usize {
        self.column_length
    }

    fn actions(&self) -> &[GameEazyMazeAction; 4] {
        &self.actions
    }

    fn transit_func(
        &self,
        state: &GameEasyMazeState,
        action: &GameEazyMazeAction,
    ) -> Result<HashMap<GameEasyMazeState, f64>, String> {
        let mut transit_probabilities = HashMap::<GameEasyMazeState, f64>::new();

        if !self.can_action_at(state) {
            return Err("Done".to_owned());
        }

        let opposite_direction = match action {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        };

        for a in self.actions() {
            let prob = if *a == *action {
                self.move_prob
            } else if *a != opposite_direction {
                (1.0 - self.move_prob) / 2.0
            } else {
                0.0
            };

            let next_state = self.try_to_move(state, a);

            let entry = transit_probabilities.entry(next_state).or_insert(0.0);

            *entry += prob;
        }

        Ok(transit_probabilities)
    }

    fn can_action_at(&self, state: &GameEasyMazeState) -> bool {
        self.grid[state.get_row() as usize][state.get_column() as usize] == MOVABLE
    }

    fn try_to_move(
        &self,
        state: &GameEasyMazeState,
        action: &GameEazyMazeAction,
    ) -> GameEasyMazeState {
        let mut next_state = {
            let mut tmp_state = state.to_owned();

            // execute an action (move)
            match action {
                Up => tmp_state.step(-1, 0),
                Down => tmp_state.step(1, 0),
                Left => tmp_state.step(0, -1),
                Right => tmp_state.step(0, 1),
            }

            tmp_state
        };

        if !(0 <= next_state.get_row()
            && next_state.get_row() < self.row_length as i32
            && 0 <= next_state.get_column()
            && next_state.get_column() < self.column_length as i32
            && self.grid[next_state.get_row() as usize][next_state.get_column() as usize] != BLOCK)
        {
            next_state = state.to_owned();
        }

        next_state
    }
}

#[cfg(test)]
mod test_game_easy_maze_environment {
    use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction::Down;
    use game::game_easy_maze::game_easy_maze_action::GameEazyMazeAction::Up;
    use game::game_easy_maze::game_easy_maze_environment::GameEazyMazeEnvironment;
    use game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral::BLOCK;
    use game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral::MOVABLE;
    use game::game_easy_maze::game_easy_maze_state::GameEasyMazeState;

    #[test]
    fn test_new() {
        let grid = vec![
            vec![MOVABLE, MOVABLE, MOVABLE],
            vec![MOVABLE, MOVABLE, MOVABLE],
            vec![MOVABLE, MOVABLE, MOVABLE],
        ];

        GameEazyMazeEnvironment::new(grid, 0.8);
    }

    #[test]
    fn test_get_movable_states() {
        let grid = vec![
            vec![MOVABLE, MOVABLE, BLOCK],
            vec![MOVABLE, MOVABLE, MOVABLE],
            vec![MOVABLE, MOVABLE, MOVABLE],
        ];

        let movable_states = GameEazyMazeEnvironment::get_movable_states(&grid);

        assert_eq!(movable_states.len(), 8);
    }

    #[test]
    fn test_transit_func() {
        let grid = vec![
            vec![MOVABLE, MOVABLE, BLOCK],
            vec![MOVABLE, MOVABLE, MOVABLE],
            vec![MOVABLE, MOVABLE, MOVABLE],
        ];

        let move_prob = 0.8;

        let env = GameEazyMazeEnvironment::new(grid, move_prob);

        let cur_state = GameEasyMazeState::new(0, 0);

        let transit_probabilities = env.transit_func(&cur_state, &Down).unwrap();

        assert_eq!(transit_probabilities.len(), 3);

        let prob_00 = transit_probabilities[&GameEasyMazeState::new(0, 0)];
        let prob_01 = transit_probabilities[&GameEasyMazeState::new(0, 1)];
        let prob_10 = transit_probabilities[&GameEasyMazeState::new(1, 0)];

        let side_prob = (1.0 - move_prob) / 2.0;

        let eps = 1e-10;

        assert!((side_prob - prob_00).abs() < eps);
        assert!((side_prob - prob_01).abs() < eps);
        assert!((move_prob - prob_10).abs() < eps);
    }
}
