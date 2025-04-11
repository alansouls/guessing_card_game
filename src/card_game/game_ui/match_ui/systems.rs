use bevy::{
    asset::{AssetServer, Assets},
    color::{palettes::css::{CRIMSON, YELLOW}, Color},
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::{Changed, With, Without},
        system::{Commands, Query, Res, ResMut, Single},
    },
    hierarchy::{BuildChildren, ChildBuild},
    input::{mouse::MouseButton, ButtonInput},
    math::{
        primitives::Annulus, Vec2
    },
    render::{
        camera::Camera,
        mesh::{Mesh, Mesh2d},
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    text::{TextColor, TextFont},
    transform::components::{GlobalTransform, Transform},
    ui::{
        widget::{Button, Text}, AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, PositionType, UiRect, Val
    },
    utils::default,
    window::Window,
};

use crate::card_game::{
    game_logic_runner::{
        components::{Card, CurrentPlayer, Guess, MaxGuess},
        events::{CardPlayed, PlayerGuessed},
    },
    game_ui::{
        DISABLED_BUTTON, NORMAL_BUTTON, TEXT_COLOR, asset_loader::AssetLoader,
        components::ButtonDisabled, match_ui::components::CardDisplay,
    },
};

use super::components::{
    AddGuessButton, CardSelected, ConfirmGuessButton, GuessUI, MatchButtonAction, MatchUI,
    OnPauseScreen, PauseButtonAction, PlayArea, PlayAreaBundle, RemoveGuessButton, VisibleCard,
};

const CARD_WIDTH: f32 = 130.0;
const CARD_HEIGHT: f32 = 202.0;

pub fn match_ui_setup(mut commands: Commands, current_player: Single<Entity, With<CurrentPlayer>>) {
    let mut entity = commands.entity(*current_player);

    entity.insert((
        Text::new("Player 1's turn"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
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

pub fn display_player_cards(
    mut commands: Commands,
    current_player_query: Query<&CurrentPlayer, Changed<CurrentPlayer>>,
    visible_cards_query: Query<(Entity, &Card), With<VisibleCard>>,
    hidden_cards_query: Query<(Entity, &Card), Without<VisibleCard>>,
    asset_server: Res<AssetServer>,
) {
    let mut inital_x = -300.0;
    const SPACING: f32 = 20.0 + CARD_WIDTH;
    for current_player in current_player_query.iter() {
        for (entity_id, card) in hidden_cards_query.iter() {
            match card.player_id {
                Some(card_player_id) => {
                    let mut entity = commands.entity(entity_id);

                    if card_player_id == current_player.0 {
                        entity.insert(CardDisplay {
                            sprite: asset_server.load_card_sprite(&card.card),
                            transform: Transform::from_xyz(inital_x, -200.0, 0.0),
                            visible: VisibleCard,
                        });
                        inital_x += SPACING;
                    }
                }
                None => continue,
            }
        }

        for (entity_id, card) in visible_cards_query.iter() {
            match card.player_id {
                Some(card_player_id) => {
                    let mut entity = commands.entity(entity_id);

                    if card_player_id != current_player.0 {
                        entity.remove::<CardDisplay>();
                    }
                }
                None => continue,
            }
        }
    }
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

            entity.insert((CardSelected {
                inital_card_position: (transform.translation.x, transform.translation.y),
            },));
        }
    }
}

pub fn unselect_card(
    mut commands: Commands,
    current_player: Single<&CurrentPlayer>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut card_query: Query<(Entity, &mut Transform, &CardSelected, &Card), With<CardSelected>>,
    play_area_query: Query<&PlayArea>,
    mut play_events: EventWriter<CardPlayed>,
) {
    if buttons.just_released(MouseButton::Left) {
        for (entity_id, mut transform, card_selected, card) in card_query.iter_mut() {
            let mut entity = commands.entity(entity_id);

            entity.remove::<CardSelected>();

            let play_area = play_area_query.single();

            if (transform.translation.x > -play_area.0 && transform.translation.x < play_area.0)
                && (transform.translation.y > -play_area.0 && transform.translation.y < play_area.0)
            {
                play_events.send(CardPlayed {
                    player_id: current_player.0,
                    card: card.card
                });
            } else {
                transform.translation.x = card_selected.inital_card_position.0;
                transform.translation.y = card_selected.inital_card_position.1;
            }
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
        (&Interaction, &MatchButtonAction, Option<&ButtonDisabled>),
        (Changed<Interaction>, With<Button>),
    >,
    mut guess: Single<&mut Guess>,
    current_player: Single<&CurrentPlayer>,
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
                    if guess.0 < 3 {
                        //TODO: Change this to inital card count
                        guess.0 += 1;
                    }
                }
                MatchButtonAction::ConfirmGuess => {
                    player_guessed_events.send(PlayerGuessed {
                        player_id: current_player.0,
                        guess: guess.0,
                    });
                }
            }
        }
    }
}

pub fn handle_guess_changed(mut guess_changed_query: Query<(&Guess, &mut Text), Changed<Guess>>) {
    for (guess, mut text) in guess_changed_query.iter_mut() {
        text.0 = format!("What's your guess: {}", guess.0);
    }
}

pub fn enable_disable_add_guess_button(
    mut commands: Commands,
    max_guess: Single<&MaxGuess>,
    guess_count_query: Query<&Guess, Changed<Guess>>,
    mut add_player_button_query: Query<
        (Entity, Option<&ButtonDisabled>, &AddGuessButton),
        With<Button>,
    >,
) {
    for guess_count in guess_count_query.iter() {
        for (entity, disabled, _) in &mut add_player_button_query {
            if guess_count.0 == max_guess.0 && disabled.is_none() {
                commands.entity(entity).insert(ButtonDisabled);
            } else if guess_count.0 < max_guess.0 && disabled.is_some() {
                commands.entity(entity).remove::<ButtonDisabled>();
            }
        }
    }
}

pub fn enable_disable_remove_guess_button(
    mut commands: Commands,
    guess_count_query: Query<&Guess, Changed<Guess>>,
    mut add_player_button_query: Query<
        (Entity, Option<&ButtonDisabled>, &RemoveGuessButton),
        With<Button>,
    >,
) {
    for guess_count in guess_count_query.iter() {
        for (entity, disabled, _) in &mut add_player_button_query {
            if guess_count.0 == 0 && disabled.is_none() {
                commands.entity(entity).insert(ButtonDisabled);
            } else if guess_count.0 > 0 && disabled.is_some() {
                commands.entity(entity).remove::<ButtonDisabled>();
            }
        }
    }
}

pub fn handle_current_player_changed(
    mut current_player_query: Query<(&CurrentPlayer, &mut Text), Changed<CurrentPlayer>>,
) {
    for (current_player, mut text) in current_player_query.iter_mut() {
        text.0 = format!("Player {}'s turn", current_player.0 + 1);
    }
}

pub fn handle_guess_current_player_changed(
    mut current_player_query: Query<&CurrentPlayer, Changed<CurrentPlayer>>,
    mut guess: Single<&mut Guess>,
) {
    for _current_player in current_player_query.iter_mut() {
        guess.0 = 0;
    }
}

pub fn setup_play_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RADIUS: f32 = 30.0;
    commands.spawn(PlayAreaBundle {
        mesh: Mesh2d(meshes.add(Annulus::new(RADIUS - 2.0, RADIUS))),
        mesh_material: MeshMaterial2d(materials.add(Color::from(CRIMSON))),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        play_area: PlayArea(RADIUS),
    });
}

pub fn highlight_play_area(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut play_area_query: Query<(&PlayArea, &mut MeshMaterial2d<ColorMaterial>)>,
    selected_card: Query<&Transform, With<CardSelected>>,
) {
    for (play_area, mesh_material) in play_area_query.iter_mut() {
        let color_material = materials.get_mut(mesh_material.0.id()).unwrap();
        match selected_card.get_single() {
            Ok(transform) => {
                if (transform.translation.x > -play_area.0 && transform.translation.x < play_area.0)
                    && (transform.translation.y > -play_area.0 && transform.translation.y < play_area.0)
                {
                    color_material.color = Color::from(YELLOW);
                } else {
                    color_material.color = Color::from(CRIMSON);
                }
            }
            Err(_) => {
                color_material.color = Color::from(CRIMSON);
            }
        }
    }
}
