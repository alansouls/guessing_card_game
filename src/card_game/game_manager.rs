use bevy::{
    app::{App, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::Plugin,
};

pub mod components;
pub mod systems;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                systems::create_deck,
                systems::shuffle_deck,
                systems::print_cards,
                systems::create_players,
                systems::distribute_cards,
                systems::print_player_hands,
                systems::print_deck_count,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                systems::execute_turn,
                systems::handle_after_turn,
                systems::print_player_hands,
                systems::print_cards_played,
            )
                .chain(),
        );
    }
}
