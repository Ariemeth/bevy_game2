use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Running,
    Paused,
}