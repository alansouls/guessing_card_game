use bevy::state::state::States;

pub mod game_manager;
pub mod game_ui;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    LocalGame,
    OnlineGame,
}