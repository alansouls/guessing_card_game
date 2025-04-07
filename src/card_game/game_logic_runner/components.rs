use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CurrentPlayer(pub usize);

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct Guess(pub usize);