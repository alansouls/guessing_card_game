use bevy::prelude::*;
use card_game::game_manager;
use card_game::game_ui;
use card_game::GameState;

mod card_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((game_ui::GameUIPlugin, game_manager::GameManagerPlugin))
        .run();
}
