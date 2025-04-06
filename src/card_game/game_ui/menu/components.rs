use bevy::ecs::component::Component;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnLocalGameScreen;


#[derive(Component)]
pub enum MenuButtonAction {
    PlayLocalGame,
    ConfirmLocalGame,
    RemoveLocalPlayer,
    AddLocalPlayer,
    PlayOnlineGame,
    ConfirmOnlineGame,
    BackToMainMenu,
    Quit,
}

#[derive(Component)]
pub struct NumberOfLocalPLayers(pub usize);

#[derive(Component)]
pub struct ButtonDisabled;

#[derive(Component)]
pub struct AddPlayerButton;

#[derive(Component)]
pub struct RemovePlayerButton;