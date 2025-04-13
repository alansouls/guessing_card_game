use bevy::{
    ecs::{bundle::Bundle, component::Component}, render::mesh::Mesh2d, sprite::{ColorMaterial, Material2d, MeshMaterial2d, Sprite}, transform::components::Transform
};

#[derive(Component)]
pub struct MatchUI;

#[derive(Component)]
pub struct CardSelected {
    pub inital_card_position: (f32, f32),
}

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
    pub sprite: Sprite,
    pub transform: Transform,
    pub visible: VisibleCard,
}

#[derive(Component)]
pub struct PlayArea(pub f32);

#[derive(Bundle)]
pub struct PlayAreaBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub play_area: PlayArea,
}

#[derive(Component)]
pub struct PlayerInfoUI(pub usize);