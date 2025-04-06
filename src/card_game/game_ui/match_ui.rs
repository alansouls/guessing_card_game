pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;

use crate::card_game::GameState;

use super::despawn_screen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MatchState {
    Enabled,
    #[default]
    Disabled,
}

pub struct GameUIMatchPlugin;

impl Plugin for GameUIMatchPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MatchState>();
    }
}
