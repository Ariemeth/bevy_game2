use bevy::prelude::{Commands, Component, Entity, Query, With};

pub mod start_menu;
pub mod primary_game_screen;

fn despawn_screen<T: Component>(mut commands: Commands, screens: Query<Entity, With<T>>) {
    for entity in &screens {
        commands.entity(entity).despawn();
    }
}