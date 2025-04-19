use bevy::ecs::{entity::Entity, system::Commands};

pub mod components;
pub mod systems;
pub mod events;

pub trait TextInputSpawner {
    fn spawn_text_input(
        &mut self,
        text: &str,
        font_size: f32,
        width: f32,
        height: f32,
    ) -> Entity;
}

impl TextInputSpawner for Commands<'_, '_> {
    fn spawn_text_input(
        &mut self,
        text: &str,
        font_size: f32,
        width: f32,
        height: f32,
    ) -> Entity {
        
    }
}