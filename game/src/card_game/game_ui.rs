use bevy::{
    app::{App, Startup, Update},
    color::Color,
    prelude::Plugin,
};
use menu::GameUIMenuPlugin;

pub mod match_ui;
pub mod menu;
pub mod asset_loader;
pub mod ui_entities;

pub mod components;
pub mod systems;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const DISABLED_TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const DISABLED_BUTTON: Color = Color::srgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUIMenuPlugin)
            .add_plugins(match_ui::GameUIMatchPlugin)
            .add_systems(Startup, systems::setup)
            .add_systems(Update, (systems::button_enabled, systems::button_system));
    }
}
