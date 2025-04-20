use bevy::{
    app::{App, Plugin, Update},
    ecs::system::EntityCommands,
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    text::{TextColor, TextFont},
    ui::{AlignItems, BackgroundColor, JustifyContent, Node, UiRect, Val, widget::Text},
    utils::default,
};
use components::{TextInput, TextInputLabel, TextInputValue};

use crate::card_game::game_ui::{NORMAL_BUTTON, TEXT_COLOR};

pub mod components;
pub mod events;
pub mod systems;

pub trait TextInputSpawner {
    fn spawn_text_input(
        &mut self,
        label: &str,
        text: &str,
        max_length: usize,
        font_size: f32,
        width: f32,
        height: f32,
    ) -> EntityCommands;
}

impl TextInputSpawner for ChildBuilder<'_> {
    fn spawn_text_input(
        &mut self,
        label: &str,
        text: &str,
        max_length: usize,
        font_size: f32,
        width: f32,
        height: f32,
    ) -> EntityCommands {
        // Common style for all buttons on the screen
        let text_input_node = Node {
            width: Val::Px(width),
            height: Val::Px(height),
            margin: UiRect::all(Val::Px(20.0)),
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        };

        let text_font = TextFont {
            font_size,
            ..default()
        };

        let mut result = self.spawn((
            TextInput {
                label: label.to_string(),
                value: text.to_string(),
                max_length,
            },
            text_input_node.clone(),
            BackgroundColor(NORMAL_BUTTON),
        ));

        result.with_children(|parent| {
            parent.spawn((
                Text::new(label),
                text_font.clone(),
                TextColor(TEXT_COLOR),
                TextInputLabel,
            ));

            parent.spawn((
                Text::new(""),
                text_font.clone(),
                TextColor(TEXT_COLOR),
                TextInputValue,
            ));
        });

        result
    }
}

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::set_text_input_active,
                systems::handle_text_key_input,
                systems::update_text_input_value,
                systems::update_text_input_label,
            ),
        );
    }
}
