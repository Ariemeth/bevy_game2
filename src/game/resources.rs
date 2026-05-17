use crate::game::balance::AUTO_CLICKER_BASE_COST;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameData {
    pub currency: f64,
    pub click_power: f64,
    pub production_rate: f64,
    pub auto_clicker_count: u32,
    pub auto_clicker_cost: f64,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            currency: 0.0,
            click_power: 1.0,
            production_rate: 0.0,
            auto_clicker_count: 0,
            auto_clicker_cost: AUTO_CLICKER_BASE_COST,
        }
    }
}
