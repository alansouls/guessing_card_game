use bevy::ecs::component::Component;

#[derive(Component)]
pub struct MatchUI;

#[derive(Component)]
pub struct CardSelected;

#[derive(Component)]
pub struct OnPauseScreen;

#[derive(Component)]
pub enum PauseButtonAction {
    ResumeGame,
    QuitToMainMenu
}

#[derive(Component)]
pub enum MatchButtonAction {
    RemoveGuess,
    AddGuess,
    ConfirmGuess
}

#[derive(Component)]
pub struct RemoveGuessButton;

#[derive(Component)]
pub struct AddGuessButton;

#[derive(Component)]
pub struct ConfirmGuessButton;

#[derive(Component)]
pub struct GuessUI;
