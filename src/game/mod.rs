pub mod events;
pub mod resources;
pub mod systems;

use crate::game::resources::GameData;
use crate::game::systems::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        app.add_observer(handle_upgrades);
        app.add_systems(Update, idle_production);
    }
}
