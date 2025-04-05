use bevy::{app::{App, Startup}, ecs::schedule::IntoSystemConfigs, prelude::Plugin};

pub mod components;
pub mod systems;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (systems::create_deck, systems::print_cards).chain());
    }
}