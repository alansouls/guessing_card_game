use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

use crate::card_game::game_ui::DISABLED_TEXT_COLOR;
use crate::card_game::GameState;

const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 8;

use super::super::{DISABLED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, TEXT_COLOR};

use super::MenuState;
use super::components::*;
use super::events::{AddPlayer, RemovePlayer};

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

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands) {
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
            OnMainMenuScreen,
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
                        Text::new("Guessing Card Game"),
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
                            MenuButtonAction::PlayLocalGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Play Local Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::PlayOnlineGame,
                            ButtonDisabled,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Play Online Game"),
                                button_text_font.clone(),
                                TextColor(DISABLED_TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                button_text_font,
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

pub fn local_game_menu_setup(mut commands: Commands) {
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
            OnLocalGameScreen,
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
                        Text::new("Number of Players: 2"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                        NumberOfLocalPLayers(2),
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
                                    MenuButtonAction::RemoveLocalPlayer,
                                    ButtonDisabled,
                                    RemovePlayerButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Remove Player"),
                                        button_text_font.clone(),
                                        TextColor(TEXT_COLOR),
                                    ));
                                });

                            parent
                                .spawn((
                                    Button,
                                    button_node.clone(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    MenuButtonAction::AddLocalPlayer,
                                    AddPlayerButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Add Player"),
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
                            MenuButtonAction::ConfirmLocalGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Start Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Back to Main Menu"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction, Option<&ButtonDisabled>),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut add_player_events: EventWriter<AddPlayer>,
    mut remove_player_events: EventWriter<RemovePlayer>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action, disabled) in &interaction_query {
        if *interaction == Interaction::Pressed && disabled.is_none() {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::PlayLocalGame => menu_state.set(MenuState::LocalGame),
                MenuButtonAction::PlayOnlineGame => menu_state.set(MenuState::OnlineGame),
                MenuButtonAction::ConfirmLocalGame => {
                    game_state.set(GameState::LocalGame);
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main);
                }
                MenuButtonAction::AddLocalPlayer => {
                    add_player_events.send(AddPlayer);
                }
                MenuButtonAction::RemoveLocalPlayer => {
                    remove_player_events.send(RemovePlayer);
                }
                _ => (),
            }
        }
    }
}

pub fn add_player(
    mut add_player_events: EventReader<AddPlayer>,
    mut player_count_query: Query<&mut NumberOfLocalPLayers>,
) {
    for _ in add_player_events.read() {
        let mut player_count = player_count_query.single_mut();
        if player_count.0 < MAX_PLAYERS {
            player_count.0 += 1;
        }
    }
}

pub fn remove_player(
    mut remove_player_events: EventReader<RemovePlayer>,
    mut player_count_query: Query<&mut NumberOfLocalPLayers>,
) {
    for _ in remove_player_events.read() {
        let mut player_count = player_count_query.single_mut();
        if player_count.0 > MIN_PLAYERS {
            player_count.0 -= 1;
        }
    }
}

pub fn update_player_count_text(
    mut player_count_query: Query<
        (&mut Text, &NumberOfLocalPLayers),
        Changed<NumberOfLocalPLayers>,
    >,
) {
    for (mut text, player_count) in &mut player_count_query {
        text.0 = format!("Number of Players: {}", player_count.0);
    }
}

pub fn enable_disable_add_player_button(
    mut commands: Commands,
    player_count_query: Query<&NumberOfLocalPLayers, Changed<NumberOfLocalPLayers>>,
    mut add_player_button_query: Query<
        (Entity, Option<&ButtonDisabled>, &AddPlayerButton),
        With<Button>,
    >,
) {
    for player_count in player_count_query.iter() {
        for (entity, disabled, _) in &mut add_player_button_query {
            if player_count.0 == MAX_PLAYERS && disabled.is_none() {
                commands.entity(entity).insert(ButtonDisabled);
            } else if player_count.0 < MAX_PLAYERS && disabled.is_some() {
                commands.entity(entity).remove::<ButtonDisabled>();
            }
        }
    }
}

pub fn enable_disable_remove_player_button(
    mut commands: Commands,
    player_count_query: Query<&NumberOfLocalPLayers, Changed<NumberOfLocalPLayers>>,
    mut remove_player_button_query: Query<
        (Entity, Option<&ButtonDisabled>, &RemovePlayerButton),
        With<Button>,
    >,
) {
    for player_count in player_count_query.iter() {
        for (entity, disabled, _) in &mut remove_player_button_query {
            if player_count.0 == MIN_PLAYERS && disabled.is_none() {
                commands.entity(entity).insert(ButtonDisabled);
            } else if player_count.0 > MIN_PLAYERS && disabled.is_some() {
                commands.entity(entity).remove::<ButtonDisabled>();
            }
        }
    }
}
