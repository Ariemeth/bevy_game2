// use bevy::prelude::*;

use bevy::prelude::*;
use crate::game::state::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), despawn_screen::<MenuUi>)
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_screen::<PauseUi>)
            .add_systems(
                Update,
                (
                    pause_on_escape.run_if(in_state(GameState::Running)),
                    handle_menu_buttons.run_if(in_state(GameState::Menu)),
                    handle_pause_buttons.run_if(in_state(GameState::Paused)),
                ),
            );
    }
}

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct PauseUi;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ResumeButton;

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MenuUi,
        children![(
            Button,
            Node {
                width: px(220.),
                height: px(72.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Srgba::hex("#2563EB").unwrap().into()),
            StartButton,
            children![(
                Text::new("Start"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            )]
        )],
    ));
}

fn setup_pause_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
        PauseUi,
        children![(
            Button,
            Node {
                width: px(220.),
                height: px(72.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Srgba::hex("#16A34A").unwrap().into()),
            ResumeButton,
            children![(
                Text::new("Resume"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            )]
        )],
    ));
}

fn handle_menu_buttons(
    mut commands: Commands,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<StartButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Srgba::hex("#1D4ED8").unwrap().into());
                next_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Srgba::hex("#3B82F6").unwrap().into());
            }
            Interaction::None => {
                *background = BackgroundColor(Srgba::hex("#2563EB").unwrap().into());
            }
        }
    }
}

fn handle_pause_buttons(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ResumeButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Srgba::hex("#15803D").unwrap().into());
                next_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Srgba::hex("#22C55E").unwrap().into());
            }
            Interaction::None => {
                *background = BackgroundColor(Srgba::hex("#16A34A").unwrap().into());
            }
        }
    }
}

fn pause_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn despawn_screen<T: Component>(mut commands: Commands, screens: Query<Entity, With<T>>) {
    for entity in &screens {
        commands.entity(entity).despawn();
    }
}
