use bevy::prelude::*;

use super::{components::ButtonDisabled, DISABLED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_enabled(
    mut removed_disabled_query: RemovedComponents<ButtonDisabled>,
    mut query_for_removed: Query<&mut BackgroundColor, With<Button>>,
) {
    // Remove the disabled state from the button if it is removed from the entity
    for entity in removed_disabled_query.read() {
        if let Ok(mut background_color) = query_for_removed.get_mut(entity) {
            *background_color = NORMAL_BUTTON.into();
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ButtonDisabled>,
        ),
        (
            Or<(Added<ButtonDisabled>, Changed<Interaction>)>,
            With<Button>,
        ),
    >,
) {
    for (interaction, mut background_color,  button_dissabled) in
        &mut interaction_query
    {
        *background_color = match (*interaction, button_dissabled) {
            (_, Some(_)) => DISABLED_BUTTON.into(),
            (Interaction::Pressed, None) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        };
    }
}