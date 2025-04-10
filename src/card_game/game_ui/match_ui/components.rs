use bevy::{
    ecs::{bundle::Bundle, component::Component},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

#[derive(Component)]
pub struct MatchUI;

#[derive(Component)]
pub struct CardSelected;

#[derive(Component)]
pub struct OnPauseScreen;

#[derive(Component)]
pub enum PauseButtonAction {
    ResumeGame,
    QuitToMainMenu,
}

#[derive(Component)]
pub enum MatchButtonAction {
    RemoveGuess,
    AddGuess,
    ConfirmGuess,
}

#[derive(Component)]
pub struct RemoveGuessButton;

#[derive(Component)]
pub struct AddGuessButton;

#[derive(Component)]
pub struct ConfirmGuessButton;

#[derive(Component)]
pub struct GuessUI;

#[derive(Component)]
pub struct VisibleCard;

#[derive(Bundle)]
pub struct CardDisplay {
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub visible: VisibleCard,
}
