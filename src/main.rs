use bevy::prelude::*;

mod card_game;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(card_game::game_manager::GameManagerPlugin)
        .run();
}
