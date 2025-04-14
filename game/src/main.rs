use bevy::prelude::*;

mod card_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Guessing Card Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(card_game::CardGamePlugin)
        .run();
}
