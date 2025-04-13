pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use components::DisplayPlayedCardTimer;
use events::{CardPlayed, GameEnded, PlayerGuessed};

use super::GameState;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MatchState {
    Paused,
    Guessing,
    Playing,
    Finished,
    DisplayingPlayedCard,
    #[default]
    Disabled,
}

pub struct GameLogicRunnerPlugin;

impl Plugin for GameLogicRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MatchState>()
            .add_event::<GameEnded>()
            .add_event::<CardPlayed>()
            .add_event::<PlayerGuessed>()
            .add_systems(
                OnEnter(GameState::LocalGameInit),
                (systems::handle_game_start, systems::spawn_cards).chain(),
            )
            .add_systems(
                Update,
                (systems::clear_cards).run_if(in_state(MatchState::DisplayingPlayedCard)),
            )
            .add_systems(
                OnExit(MatchState::DisplayingPlayedCard),
                systems::spawn_cards,
            )
            .add_systems(
                Update,
                (systems::handle_player_guess, systems::handle_card_played)
                    .run_if(in_state(GameState::LocalGame)),
            );
    }
}
