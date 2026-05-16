
mod game;
mod ui;
mod balance;

use bevy::prelude::*;
use crate::game::GamePlugin;
use crate::ui::UiPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, UiPlugin))
        .run()
}
