use bevy::{
    app::{App, Plugin},
    ecs::system::Resource,
    state::{app::AppExtStates, state::States},
};

use card_game_logic::game_logic::local::LocalGameLogic;
use game_logic_runner::game_logic_facade::GameLogicFacade;

pub mod game_logic_runner;
pub mod game_ui;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    LocalGameInit,
    LocalGame,
    OnlineGameInit,
    OnlineGame,
}

#[derive(Resource)]
pub struct GameSettings {
    pub player_count: usize,
    pub inital_card_count: usize,
    pub online_player_name: String,
    pub online_room_name: String,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_count: 4,
            inital_card_count: 3,
            online_player_name: "Player".to_string(),
            online_room_name: "Room".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct GameLogicRes(pub GameLogicFacade);

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(GameSettings::default())
            .insert_resource(GameLogicRes(GameLogicFacade::new()))
            .add_plugins(game_logic_runner::GameLogicRunnerPlugin)
            .add_plugins(game_ui::GameUIPlugin);
    }
}
