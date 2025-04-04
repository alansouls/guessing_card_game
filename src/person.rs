use bevy::prelude::*;

pub mod commands;
pub mod queries;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);