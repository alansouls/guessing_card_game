use bevy::ecs::component::Component;

#[derive(Component)]
pub struct TextInput {
    pub label: String,
    pub value: String,
}
