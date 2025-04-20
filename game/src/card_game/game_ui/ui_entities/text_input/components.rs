use bevy::{
    ecs::component::Component,
    ui::{FocusPolicy, Interaction, Node},
};

#[derive(Component)]
#[require(Node, FocusPolicy(|| FocusPolicy::Block), Interaction)]
pub struct TextInput {
    pub label: String,
    pub value: String,
    pub max_length: usize,
}

#[derive(Component)]
pub struct TextInputLabel;

#[derive(Component)]
pub struct TextInputValue;

#[derive(Component)]
pub struct TextInputActive;
