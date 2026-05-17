use crate::game::state::GameState;
use bevy::prelude::*;
use crate::ui::despawn_screen;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), despawn_screen::<StartMenuUI>)
            .add_systems(
                Update,
                handle_start_button.run_if(in_state(GameState::Menu)),
            );
    }
}

#[derive(Component)]
struct StartMenuUI;

#[derive(Component)]
struct StartButton;

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
        StartMenuUI,
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

fn handle_start_button(
    mut button: Single<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >, mut next_state: ResMut<NextState<GameState>>,
){
    match *button.0 {
        Interaction::Pressed => {
            *button.1 = BackgroundColor(Srgba::hex("#1D4ED8").unwrap().into());
            next_state.set(GameState::Running);
        }
        Interaction::Hovered => {
            *button.1 = BackgroundColor(Srgba::hex("#3B82F6").unwrap().into());
        }
        Interaction::None => {
            *button.1 = BackgroundColor(Srgba::hex("#2563EB").unwrap().into());
        }
    }
}


