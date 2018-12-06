extern crate artificial_intelligence_gym;

use artificial_intelligence_gym::game::game_easy_maze::game_easy_maze_environment::GameEazyMazeEnvironment;
use artificial_intelligence_gym::game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral::BLOCK;
use artificial_intelligence_gym::game::game_easy_maze::game_easy_maze_literal::GameEasyMazeLiteral::MOVABLE;

fn main() {
    let grid = vec![
        vec![MOVABLE, MOVABLE, BLOCK],
        vec![MOVABLE, MOVABLE, MOVABLE],
        vec![MOVABLE, MOVABLE, MOVABLE]
    ];

    let env = GameEazyMazeEnvironment::new(grid, 0.8);

    print!("{:?}", env);
}