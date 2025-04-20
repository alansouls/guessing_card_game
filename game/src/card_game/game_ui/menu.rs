pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use events::{AddPlayer, RemovePlayer};
use systems::{
    add_player, cleanup_local_game_menu, cleanup_main_menu, cleanup_online_game_menu,
    enable_disable_add_player_button, enable_disable_remove_player_button, local_game_menu_setup,
    main_menu_setup, menu_action, menu_setup, online_game_menu_setup, remove_player,
    update_player_count_text,
};

use crate::card_game::GameState;

use super::systems::despawn_screen;

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
            .add_systems(OnExit(GameState::Menu), despawn_screen::<components::OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), cleanup_main_menu)
            .add_systems(OnEnter(MenuState::LocalGame), local_game_menu_setup)
            .add_systems(OnExit(MenuState::LocalGame), cleanup_local_game_menu)
            .add_systems(OnEnter(MenuState::OnlineGame), online_game_menu_setup)
            .add_systems(OnExit(MenuState::OnlineGame), cleanup_online_game_menu)
            .add_event::<AddPlayer>()
            .add_event::<RemovePlayer>()
            .add_systems(
                Update,
                (
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
