mod balance;
mod game;
mod ui;

use crate::game::GamePlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, UiPlugin))
        .run()
}
