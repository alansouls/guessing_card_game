use bevy::ecs::event::Event;

#[derive(Event)]
pub struct RemovePlayer;

#[derive(Event)]
pub struct AddPlayer;