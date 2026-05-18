use bevy::prelude::*;

#[derive(Event)]
pub struct ClickEvent;

#[derive(Event)]
pub enum UpgradeEvent {
    AutoClicker,
}
