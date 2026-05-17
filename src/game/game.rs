use bevy::app::{App, Plugin, Update};
use bevy::prelude::{On, Res, ResMut, Time};
use crate::game::balance::{AUTO_CLICKER_COST_SCALING, AUTO_CLICKER_PRODUCTION};
use crate::game::events::UpgradeEvent;
use crate::game::resources::GameData;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        app.add_observer(handle_upgrades);
        app.add_systems(Update, idle_production);
    }
}

pub fn idle_production(time: Res<Time>, mut game_data: ResMut<GameData>) {
    game_data.currency += game_data.production_rate * time.delta_secs_f64();
}

pub fn handle_upgrades(trigger: On<UpgradeEvent>, mut game_data: ResMut<GameData>) {
    match *trigger {
        UpgradeEvent::AutoClicker => {
            if game_data.currency >= game_data.auto_clicker_cost {
                game_data.currency -= game_data.auto_clicker_cost;
                game_data.auto_clicker_count += 1;
                game_data.production_rate += AUTO_CLICKER_PRODUCTION;
                game_data.auto_clicker_cost *= AUTO_CLICKER_COST_SCALING;
            }
        }
    }
}