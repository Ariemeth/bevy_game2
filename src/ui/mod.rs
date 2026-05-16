pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::ui::systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, (update_ui, handle_click));
    }
}
