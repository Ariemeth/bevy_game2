pub mod components;
pub mod systems;

use crate::ui::systems::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, (update_ui, handle_click));
    }
}
