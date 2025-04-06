use bevy::{app::{App, Plugin}, ecs::system::Resource, state::{app::AppExtStates, state::States}};
use game_logic::{local::LocalGameLogic, GameLogic};

pub mod game_logic;
pub mod game_logic_runner;
pub mod game_ui;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    LocalGame,
    OnlineGame,
}

#[derive(Resource)]
pub struct GameSettings {
    pub player_count: usize,
    pub inital_card_count: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_count: 4,
            inital_card_count: 3,
        }
    }
}

#[derive(Resource)]
pub struct LocalGameLogicRes(pub LocalGameLogic);

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(game_ui::GameUIPlugin)
            .insert_resource(GameSettings::default())
            .insert_resource(LocalGameLogicRes(LocalGameLogic::default()));
    }
}
