pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use components::{OnLocalGameScreen, OnMainMenuScreen};
use events::{AddPlayer, RemovePlayer};
use systems::{
    add_player, button_enabled, button_system, enable_disable_add_player_button,
    enable_disable_remove_player_button, local_game_menu_setup, main_menu_setup, menu_action,
    menu_setup, remove_player, update_player_count_text,
};

use crate::card_game::GameState;

use super::despawn_screen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    LocalGame,
    OnlineGame,
    #[default]
    Disabled,
}

pub struct GameUIMenuPlugin;

impl Plugin for GameUIMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::LocalGame), local_game_menu_setup)
            .add_systems(
                OnExit(MenuState::LocalGame),
                despawn_screen::<OnLocalGameScreen>,
            )
            .add_event::<AddPlayer>()
            .add_event::<RemovePlayer>()
            .add_systems(
                Update,
                (
                    button_enabled,
                    button_system,
                    menu_action,
                    add_player,
                    remove_player,
                    update_player_count_text,
                    enable_disable_add_player_button,
                    enable_disable_remove_player_button,
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}
