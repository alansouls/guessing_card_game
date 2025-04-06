pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;

use crate::card_game::GameState;

use super::despawn_screen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MatchState {
    Paused,
    Playing,
    Finished,
    #[default]
    Disabled,
}

pub struct GameUIMatchPlugin;

impl Plugin for GameUIMatchPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MatchState>()
            .add_systems(OnEnter(GameState::LocalGame), systems::match_setup)
            .add_systems(OnEnter(MatchState::Playing), systems::match_ui_setup)
            .add_systems(OnEnter(MatchState::Paused), systems::pause_setup)
            .add_systems(
                Update,
                (systems::select_card, systems::unselect_card, systems::move_card).run_if(in_state(MatchState::Playing)),
            )
            .add_systems(
                OnExit(MatchState::Paused),
                despawn_screen::<components::OnPauseScreen>,
            )
            .add_systems(
                OnExit(GameState::LocalGame),
                despawn_screen::<components::MatchUI>,
            );
    }
}
