pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;

use crate::card_game::{GameState, game_logic_runner::MatchState};

use super::systems::despawn_screen;

pub struct GameUIMatchPlugin;

impl Plugin for GameUIMatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LocalGame), systems::match_ui_setup)
            .add_systems(OnEnter(GameState::OnlineGame), systems::match_ui_setup)
            .add_systems(OnEnter(MatchState::Guessing), systems::guess_ui_setup)
            .add_systems(OnEnter(MatchState::Paused), systems::pause_setup)
            .add_systems(OnEnter(MatchState::Playing), systems::setup_play_area)
            .add_systems(
                Update,
                (
                    systems::handle_guess_action,
                    systems::enable_disable_add_guess_button,
                    systems::enable_disable_remove_guess_button,
                    systems::handle_guess_changed,
                    systems::handle_guess_current_player_changed,
                    systems::display_player_cards,
                )
                    .run_if(in_state(MatchState::Guessing)),
            )
            .add_systems(
                Update,
                (
                    systems::handle_current_player_changed,
                    systems::player_info_ui_setup,
                    systems::player_info_ui_update,
                )
                    .chain()
                    .run_if(in_state(GameState::LocalGame)),
            )
            .add_systems(
                Update,
                (
                    systems::display_player_cards,
                    systems::select_card,
                    systems::unselect_card,
                    systems::move_card,
                    systems::highlight_play_area,
                    systems::adjust_top_played_card,
                )
                    .chain()
                    .run_if(in_state(MatchState::Playing)),
            )
            .add_systems(
                OnExit(MatchState::Paused),
                despawn_screen::<components::OnPauseScreen>,
            )
            .add_systems(
                OnExit(MatchState::Guessing),
                despawn_screen::<components::GuessUI>,
            )
            .add_systems(
                OnExit(GameState::LocalGame),
                despawn_screen::<components::MatchUI>,
            )
            .add_systems(
                OnExit(GameState::OnlineGame),
                despawn_screen::<components::MatchUI>,
            );
    }
}
