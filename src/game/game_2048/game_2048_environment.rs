extern crate rand;

use game::game_2048::game_2048_action::Game2048Action;
use game::game_2048::game_2048_state::Game2048State;
use game::game_mdp::GameMdp;

use self::rand::Rng;

#[derive(Debug)]
struct Game2048Environment {}

impl Game2048Environment {
    #[allow(dead_code)]
    pub fn init() -> Game2048State {
        let mut state = Game2048State::new();

        // TODO: more specific initialization
        let size = Game2048State::get_board_size();
        state.set_element(size / 2, size / 2, 2);
        Game2048Environment::set_randomly(&mut state);

        state
    }

    fn move_up_state(state: &mut Game2048State) {
        let size = Game2048State::get_board_size();

        for j in 0..size {
            for i in 1..size {
                for k in (1..=i).rev() {
                    if state.is_empty(k, j) {
                        break;
                    }

                    if state.is_same(k - 1, j, k, j) {
                        state.double(k - 1, j);
                        state.set_element(k, j, Game2048State::get_empty_literal());
                    } else if state.is_empty(k - 1, j) {
                        state.slide(k - 1, j, k, j);
                    }
                }
            }
        }
    }

    fn move_down_state(state: &mut Game2048State) {
        let size = Game2048State::get_board_size();

        for j in 0..size {
            for i in (0..(size - 1)).rev() {
                for k in i..(size - 1) {
                    if state.is_empty(k, j) {
                        break;
                    }

                    if state.is_same(k + 1, j, k, j) {
                        state.double(k + 1, j);
                        state.set_element(k, j, Game2048State::get_empty_literal());
                    } else if state.is_empty(k + 1, j) {
                        state.slide(k + 1, j, k, j);
                    }
                }
            }
        }
    }

    fn move_left_state(state: &mut Game2048State) {
        let size = Game2048State::get_board_size();

        for i in 0..size {
            for j in 1..size {
                for k in (1..=j).rev() {
                    if state.is_empty(i, k) {
                        break;
                    }

                    if state.is_same(i, k - 1, i, k) {
                        state.double(i, k - 1);
                        state.set_element(i, k, Game2048State::get_empty_literal());
                    } else if state.is_empty(i, k - 1) {
                        state.slide(i, k - 1, i, k);
                    }
                }
            }
        }
    }

    fn move_right_state(state: &mut Game2048State) {
        let size = Game2048State::get_board_size();

        for i in 0..size {
            for j in (0..(size - 1)).rev() {
                for k in j..(size - 1) {
                    if state.is_empty(i, k) {
                        break;
                    }

                    if state.is_same(i, k + 1, i, k) {
                        state.double(i, k + 1);
                        state.set_element(i, k, Game2048State::get_empty_literal());
                    } else if state.is_empty(i, k + 1) {
                        state.slide(i, k + 1, i, k);
                    }
                }
            }
        }
    }

    fn set_randomly(state: &mut Game2048State) -> bool {
        if !state.has_empty() {
            return false;
        };

        let mut rng = rand::thread_rng();

        let board_size = Game2048State::get_board_size() as u32;

        let size = board_size.pow(2u32);

        loop {
            let pos = rng.gen_range(0, size);

            let i = (pos / board_size) as usize;
            let j = (pos % board_size) as usize;

            if state.is_empty(i, j) {
                let new_elem = Game2048Environment::do_sample();
                println!("set: {:?} {:?} : {:?}", i, j, new_elem);
                state.set_element(i, j, new_elem);
                break;
            }
        }

        true
    }

    fn do_sample() -> i32 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() >= 0.9 {
            4
        } else {
            2
        }
    }

    // TODO: find a way to only clone a once
    fn check_movable(state: &Game2048State) -> bool {
        {
            let mut up = state.clone();
            Game2048Environment::move_up_state(&mut up);
            if up != *state {
                return true;
            }
        }

        {
            let mut down = state.clone();
            Game2048Environment::move_down_state(&mut down);
            if down != *state {
                return true;
            }
        }

        {
            let mut left = state.clone();
            Game2048Environment::move_up_state(&mut left);
            if left != *state {
                return true;
            }
        }

        {
            let mut right = state.clone();
            Game2048Environment::move_up_state(&mut right);
            if right != *state {
                return true;
            }
        }

        false
    }

    #[allow(dead_code)]
    fn check_game_set(state: &Game2048State) -> bool {
        !Game2048Environment::check_movable(state)
    }
}

impl GameMdp for Game2048Environment {
    type Action = Game2048Action;
    type State = Game2048State;

    fn proceed_game(state: &Self::State, action: &Self::Action) -> (f64, Self::State) {
        let mut next_state = state.clone();
        match action {
            Game2048Action::Up => Game2048Environment::move_up_state(&mut next_state),
            Game2048Action::Down => Game2048Environment::move_down_state(&mut next_state),
            Game2048Action::Right => Game2048Environment::move_right_state(&mut next_state),
            Game2048Action::Left => Game2048Environment::move_left_state(&mut next_state),
        }

        Game2048Environment::set_randomly(&mut next_state);

        (1.0f64, next_state)
    }
}

#[cfg(test)]
mod tests_game_2048_environment {
    use game::game_2048::game_2048_action::Game2048Action;
    use game::game_2048::game_2048_environment::Game2048Environment;
    use game::game_mdp::GameMdp;

    #[test]
    fn test_move() {
        let mut state = Game2048Environment::init();

        println!("{}", &state);

        if Game2048Environment::check_game_set(&state) {
            println!("Game Set!");
            return;
        }

        for i in 1..500 {
            println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

            println!("{}", &state);

            let action = match (i as u32) % 4 {
                0 => Game2048Action::Up,
                1 => Game2048Action::Down,
                2 => Game2048Action::Right,
                _ => Game2048Action::Left,
            };

            println!("{:?}", &action);

            state = Game2048Environment::proceed_game(&state, &action).1;

            println!("{}", &state);

            if Game2048Environment::check_game_set(&state) {
                println!("Game Set!");
                break;
            }

            println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        }
    }

    #[test]
    fn test_impl() {
        let mut state = Game2048Environment::init();

        println!("{}", &state);

        state.set_element(0, 3, 4);
        state.set_element(1, 3, 4);
        state.set_element(2, 3, 16);
        state.set_element(3, 3, 16);

        println!("{}", &state);

        Game2048Environment::move_down_state(&mut state);

        println!("{}", &state);
    }
}
