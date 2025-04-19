use bevy::{ecs::{
    entity::Entity,
    query::Added,
    system::{Commands, Query},
}, hierarchy::BuildChildren, ui::widget::Text};

use super::components::TextInput;

pub fn setup_text_input(
    mut commands: Commands,
    added_text_input: Query<(Entity, &TextInput), Added<TextInput>>,
) {
    for (entity, text_input) in added_text_input.iter() {
        commands.entity(entity).insert((
            Tex
        ));
    }
}
