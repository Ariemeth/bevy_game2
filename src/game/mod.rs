pub mod resources;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use crate::game::resources::GameData;
use crate::game::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        app.add_observer(handle_upgrades);
        app.add_systems(Update, idle_production);
    }
}
