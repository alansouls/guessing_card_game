use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Changed, With, Without},
        system::{Commands, Query, Single},
    },
    hierarchy::Children,
    input::keyboard::KeyboardInput,
    ui::{Interaction, widget::Text},
};

use super::components::{TextInput, TextInputActive, TextInputLabel, TextInputValue};

pub fn set_text_input_active(
    mut commands: Commands,
    mut text_input_query: Query<
        (Entity, &Interaction, &TextInput),
        (Changed<Interaction>, Without<TextInputActive>),
    >,
    active_text_input: Option<Single<Entity, With<TextInputActive>>>,
) {
    for (entity, interaction, _) in &mut text_input_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(active_entity) = active_text_input.as_ref() {
                    commands.entity(**active_entity).remove::<TextInputActive>();
                }
                commands.entity(entity).insert(TextInputActive);
            }
            _ => (),
        }
    }
}

pub fn handle_text_key_input(
    mut key_pressed: EventReader<KeyboardInput>,
    mut text_input_active_query: Query<(&mut TextInput, &TextInputActive)>,
) {
    for event in key_pressed.read() {
        if event.state.is_pressed() {
            for (mut text_input, _) in &mut text_input_active_query {
                match event.key_code {
                    bevy::input::keyboard::KeyCode::Backspace => {
                        text_input.value.pop();
                    }
                    _ => {
                        if text_input.value.len() < text_input.max_length {
                            match key_code_to_char(event.key_code) {
                                Some(c) => {
                                    text_input.value.push(c);
                                }
                                None => (),
                            };
                        }
                    }
                }
            }
        }
    }
}

pub fn update_text_input_value(
    text_input_query: Query<(&Children, &TextInput), Changed<TextInput>>,
    mut text_query: Query<&mut Text, With<TextInputValue>>,
) {
    for (text_input_children, text_input) in &text_input_query {
        for child in text_input_children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.0 = text_input.value.clone();
            }
        }
    }
}

pub fn update_text_input_label(
    text_input_query: Query<(&Children, &TextInput), Changed<TextInput>>,
    mut text_query: Query<&mut Text, With<TextInputLabel>>,
) {
    for (text_input_children, text_input) in &text_input_query {
        for child in text_input_children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.0 = text_input.label.clone();
            }
        }
    }
}

fn key_code_to_char(keycode: bevy::input::keyboard::KeyCode) -> Option<char> {
    match keycode {
        bevy::input::keyboard::KeyCode::KeyA => Some('a'),
        bevy::input::keyboard::KeyCode::KeyB => Some('b'),
        bevy::input::keyboard::KeyCode::KeyC => Some('c'),
        bevy::input::keyboard::KeyCode::KeyD => Some('d'),
        bevy::input::keyboard::KeyCode::KeyE => Some('e'),
        bevy::input::keyboard::KeyCode::KeyF => Some('f'),
        bevy::input::keyboard::KeyCode::KeyG => Some('g'),
        bevy::input::keyboard::KeyCode::KeyH => Some('h'),
        bevy::input::keyboard::KeyCode::KeyI => Some('i'),
        bevy::input::keyboard::KeyCode::KeyJ => Some('j'),
        bevy::input::keyboard::KeyCode::KeyK => Some('k'),
        bevy::input::keyboard::KeyCode::KeyL => Some('l'),
        bevy::input::keyboard::KeyCode::KeyM => Some('m'),
        bevy::input::keyboard::KeyCode::KeyN => Some('n'),
        bevy::input::keyboard::KeyCode::KeyO => Some('o'),
        bevy::input::keyboard::KeyCode::KeyP => Some('p'),
        bevy::input::keyboard::KeyCode::KeyQ => Some('q'),
        bevy::input::keyboard::KeyCode::KeyR => Some('r'),
        bevy::input::keyboard::KeyCode::KeyS => Some('s'),
        bevy::input::keyboard::KeyCode::KeyT => Some('t'),
        bevy::input::keyboard::KeyCode::KeyU => Some('u'),
        bevy::input::keyboard::KeyCode::KeyV => Some('v'),
        bevy::input::keyboard::KeyCode::KeyW => Some('w'),
        bevy::input::keyboard::KeyCode::KeyX => Some('x'),
        bevy::input::keyboard::KeyCode::KeyY => Some('y'),
        bevy::input::keyboard::KeyCode::KeyZ => Some('z'),
        bevy::input::keyboard::KeyCode::Space => Some(' '),
        _ => None,
    }
}
