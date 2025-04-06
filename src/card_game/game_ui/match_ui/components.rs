use bevy::ecs::component::Component;


#[derive(Component)]
pub struct CurrentPlayer(pub i8);

#[derive(Component)]
pub struct MatchUI;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct CardSelected;

#[derive(Component)]
pub struct OnPauseScreen;

#[derive(Component)]
pub enum PauseButtonAction {
    ResumeGame,
    QuitToMainMenu
}

