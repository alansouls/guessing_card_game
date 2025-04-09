use bevy::{
    asset::Assets,
    color::{Color, palettes::css::CRIMSON},
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::{Added, Changed, With, Without},
        system::{Commands, Query, Res, ResMut, Single},
    },
    hierarchy::{BuildChildren, ChildBuild},
    input::{ButtonInput, mouse::MouseButton},
    math::{Vec2, primitives::Rectangle},
    render::{
        camera::Camera,
        mesh::{Mesh, Mesh2d},
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    text::{TextColor, TextFont},
    transform::components::{GlobalTransform, Transform},
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node,
        PositionType, UiRect, Val,
        widget::{Button, Text},
    },
    utils::default,
    window::Window,
};

use crate::card_game::{
    game_logic_runner::{
        components::{Card, CurrentPlayer, Guess},
        events::PlayerGuessed,
    },
    game_ui::{DISABLED_BUTTON, NORMAL_BUTTON, TEXT_COLOR, components::ButtonDisabled},
};

use super::components::{
    AddGuessButton, CardSelected, ConfirmGuessButton, GuessUI, MatchButtonAction, MatchUI,
    OnPauseScreen, PauseButtonAction, RemoveGuessButton,
};

const CARD_WIDTH: f32 = 125.0;
const CARD_HEIGHT: f32 = 200.0;

pub fn add_cards_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    card_query: Query<Entity, Added<Card>>,
) {
    let mut inital_x = -300.0;
    const SPACING: f32 = 20.0 + CARD_WIDTH;
    for entity_id in card_query.iter() {
        println!("Adding meshes to: {:?}", entity_id);
        let mut entity = commands.entity(entity_id);

        entity.insert((
            Mesh2d(meshes.add(Rectangle::new(CARD_WIDTH, CARD_HEIGHT))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(inital_x, -200.0, 0.0),
            inital_x += SPACING,
        ));
    }
}

pub fn match_ui_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CARD_WIDTH, CARD_HEIGHT))),
        // MeshMaterial2d(materials.add(Color::WHITE)),
        MatchUI,
    ));

    commands.spawn((
        Text::new("Player 1's turn"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        CurrentPlayer(0),
        MatchUI,
    ));
}

pub fn guess_ui_setup(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 25.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GuessUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("What's your guess: 0"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                        Guess(0),
                    ));

                    parent
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Button,
                                    button_node.clone(),
                                    BackgroundColor(DISABLED_BUTTON),
                                    MatchButtonAction::RemoveGuess,
                                    ButtonDisabled,
                                    RemoveGuessButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("-"),
                                        button_text_font.clone(),
                                        TextColor(TEXT_COLOR),
                                    ));
                                });

                            parent
                                .spawn((
                                    Button,
                                    button_node.clone(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    MatchButtonAction::AddGuess,
                                    AddGuessButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("+"),
                                        button_text_font.clone(),
                                        TextColor(TEXT_COLOR),
                                    ));
                                });
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MatchButtonAction::ConfirmGuess,
                            ConfirmGuessButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("GUESS!"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

pub fn pause_setup(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 25.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnPauseScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Pause"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            PauseButtonAction::ResumeGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Resume"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            PauseButtonAction::QuitToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit to Main Menu"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

pub fn select_card(
    mut commands: Commands,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    card_query: Query<(Entity, &Card, &Transform), Without<CardSelected>>,
) {
    let point = match get_mouse_position(camera_query, windows) {
        Some(point) => point,
        None => return,
    };

    if buttons.just_pressed(MouseButton::Left) {
        for (entity_id, _, transform) in card_query.iter() {
            let mut entity = commands.entity(entity_id);

            if point.x < transform.translation.x - CARD_WIDTH / 2.0
                || point.x > transform.translation.x + CARD_WIDTH / 2.0
                || point.y < transform.translation.y - CARD_HEIGHT / 2.0
                || point.y > transform.translation.y + CARD_HEIGHT / 2.0
            {
                continue;
            }

            entity.insert(CardSelected);
        }
    }
}

pub fn unselect_card(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    card_query: Query<Entity, With<CardSelected>>,
) {
    if buttons.just_released(MouseButton::Left) {
        for entity_id in card_query.iter() {
            let mut entity = commands.entity(entity_id);

            entity.remove::<CardSelected>();
        }
    }
}

pub fn move_card(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut card_query: Query<(&CardSelected, &mut Transform)>,
) {
    let point = match get_mouse_position(camera_query, windows) {
        Some(point) => point,
        None => return,
    };

    for (_, mut transform) in card_query.iter_mut() {
        // Move the card to the cursor position
        transform.translation.x = point.x;
        transform.translation.y = point.y;
    }
}

fn get_mouse_position(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return None;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return None;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return None;
    };

    Some(point)
}

pub fn handle_guess_action(
    interaction_query: Query<
        (
            &Interaction,
            &MatchButtonAction,
            Option<&ButtonDisabled>
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut guess: Single<&mut Guess>,
    mut player_guessed_events: EventWriter<PlayerGuessed>,
) {
    for (interaction, button_action, disabled) in &interaction_query {
        if *interaction == Interaction::Pressed && disabled.is_none() {
            match button_action {
                MatchButtonAction::RemoveGuess => {
                    if guess.0 > 0 {
                        guess.0 -= 1;
                    }
                }
                MatchButtonAction::AddGuess => {
                    if guess.0 < 3 { //TODO: Change this to inital card count
                        guess.0 += 1;
                    }
                }
                MatchButtonAction::ConfirmGuess => {
                    player_guessed_events.send(PlayerGuessed {
                        player_id: guess.0,
                        guess: guess.0,
                    });
                }
            }
        }
    }
}
