use bevy::prelude::*;

use crate::card_game::game_ui::components::ButtonDisabled;
use crate::card_game::game_ui::DISABLED_TEXT_COLOR;
use crate::card_game::GameState;

const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 8;

// Updated UI constants
const MENU_BACKGROUND: Color = Color::srgb(0.4, 0.1, 0.2); // Burgundy-like
const MENU_PANEL_BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.15); // Dark blue-gray
const MENU_TITLE_FONT_SIZE: f32 = 72.0;
const MENU_PANEL_PADDING: f32 = 30.0;

use super::super::{DISABLED_BUTTON, NORMAL_BUTTON, TEXT_COLOR};

use super::MenuState;
use super::components::*;
use super::events::{AddPlayer, RemovePlayer};

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(320.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 28.0,
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
            BackgroundColor(MENU_BACKGROUND),
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // Main menu panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(MENU_PANEL_PADDING)),
                        ..default()
                    },
                    BackgroundColor(MENU_PANEL_BACKGROUND),
                ))
                .with_children(|parent| {
                    // Title with enhanced styling
                    parent.spawn((
                        Text::new("Guessing Card Game"),
                        TextFont {
                            font_size: MENU_TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect { 
                                bottom: Val::Px(60.0),
                                top: Val::Px(20.0),
                                left: Val::Px(50.0),
                                right: Val::Px(50.0),
                            },
                            ..default()
                        },
                    ));

                    // Play Local Game button
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

                    // Play Online Game button
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(DISABLED_BUTTON),
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

                    // Quit button
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
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 28.0,
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
            BackgroundColor(MENU_BACKGROUND),
            OnLocalGameScreen,
        ))
        .with_children(|parent| {
            // Local game menu panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(MENU_PANEL_PADDING)),
                        ..default()
                    },
                    BackgroundColor(MENU_PANEL_BACKGROUND),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Local Game Setup"),
                        TextFont {
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect { 
                                bottom: Val::Px(40.0),
                                top: Val::Px(10.0),
                                left: Val::Px(20.0),
                                right: Val::Px(20.0),
                            },
                            ..default()
                        },
                    ));

                    // Player count with stylized container
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(340.0),
                                padding: UiRect::all(Val::Px(15.0)),
                                margin: UiRect { 
                                    bottom: Val::Px(30.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.3, 0.2)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Number of Players: 2"),
                                TextFont {
                                    font_size: 34.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                NumberOfLocalPLayers(2),
                            ));
                        });

                    // Player count controls
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect { bottom: Val::Px(30.0), ..default() },
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            // Remove Player button
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

                            // Add Player button
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

                    // Start Game button
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
                                Node {
                                    padding: UiRect { 
                                        left: Val::Px(20.0),
                                        right: Val::Px(20.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });

                    // Back to Main Menu button
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
    interaction_query: Query<(
        &Interaction,
        &MenuButtonAction,
        Option<&ButtonDisabled>,
    ), (Changed<Interaction>, With<Button>)>,
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
                    menu_state.set(MenuState::Disabled);
                    game_state.set(GameState::LocalGameInit);
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
    mut player_count_query: Query<(
        &mut Text,
        &NumberOfLocalPLayers,
    ), Changed<NumberOfLocalPLayers>>,
) {
    for (mut text, player_count) in &mut player_count_query {
        text.0 = format!("Number of Players: {}", player_count.0);
    }
}

pub fn enable_disable_add_player_button(
    mut commands: Commands,
    player_count_query: Query<&NumberOfLocalPLayers, Changed<NumberOfLocalPLayers>>,
    mut add_player_button_query: Query<(
        Entity,
        Option<&ButtonDisabled>,
        &AddPlayerButton,
    ), With<Button>>,
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
    mut remove_player_button_query: Query<(
        Entity,
        Option<&ButtonDisabled>,
        &RemovePlayerButton,
    ), With<Button>>,
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
