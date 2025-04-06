use bevy::{
    app::{App, Startup}, color::Color, core_pipeline::core_2d::Camera2d, ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    }, hierarchy::DespawnRecursiveExt, prelude::Plugin
};
use menu::GameUIMenuPlugin;

pub mod menu;
pub mod match_ui;

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
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
