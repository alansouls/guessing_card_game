use bevy::prelude::*;

mod card_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(card_game::CardGamePlugin)
        .run();
}
