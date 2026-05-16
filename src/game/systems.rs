use crate::balance::{AUTO_CLICKER_COST_SCALING, AUTO_CLICKER_PRODUCTION};
use crate::game::events::UpgradeEvent;
use crate::game::resources::GameData;
use bevy::prelude::*;

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
