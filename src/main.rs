use bevy::prelude::*;
use bevy_game2::{GamePlugin, PrimaryGameUIPlugin, MenuPlugin, GameState};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((GamePlugin, PrimaryGameUIPlugin, MenuPlugin))
        .run()
}
