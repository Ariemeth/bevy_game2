use bevy::prelude::*;
use bevy_game2::{GamePlugin, PrimaryGameUIPlugin, StartMenuPlugin, GameState};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((GamePlugin, PrimaryGameUIPlugin, StartMenuPlugin))
        .run()
}
