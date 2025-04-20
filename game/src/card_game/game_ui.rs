use bevy::{
    app::{App, Startup, Update},
    color::Color,
    prelude::Plugin,
};
use match_ui::GameUIMatchPlugin;
use menu::GameUIMenuPlugin;
use ui_entities::text_input::TextInputPlugin;

pub mod asset_loader;
pub mod match_ui;
pub mod menu;
pub mod ui_entities;

pub mod components;
pub mod systems;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const DISABLED_BUTTON: Color = Color::srgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUIMenuPlugin)
            .add_plugins(GameUIMatchPlugin)
            .add_plugins(TextInputPlugin)
            .add_systems(Startup, systems::setup)
            .add_systems(Update, (systems::button_enabled, systems::button_system));
    }
}
