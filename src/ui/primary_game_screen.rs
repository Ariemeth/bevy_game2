use crate::game::events::UpgradeEvent;
use crate::game::resources::GameData;
use crate::game::state::GameState;
use bevy::prelude::*;

pub struct PrimaryGameScreenPlugin;

impl Plugin for PrimaryGameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_ui))
            .add_systems(OnEnter(GameState::Running), show_ui)
            .add_systems(OnEnter(GameState::Menu), hide_ui)
            .add_systems(
                Update,
                (
                    update_ui,
                    handle_click_button.run_if(in_state(GameState::Running)),
                    handle_upgrade_button.run_if(in_state(GameState::Running)),
                ),
            );
    }
}

#[derive(Component)]
pub struct PrimaryGameUi;

#[derive(Component)]
pub struct CurrencyText;

#[derive(Component)]
pub struct ClickButton;

#[derive(Component)]
pub struct UpgradeButton;

#[derive(Component)]
pub struct UpgradeText;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_ui(mut commands: Commands, game_data: Res<GameData>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .insert((PrimaryGameUi, Visibility::Hidden))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Currency: 0"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                CurrencyText,
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ClickButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Click!"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.3, 0.15)),
                    UpgradeButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(format!(
                            "Buy AutoClicker ({:.1})",
                            game_data.auto_clicker_cost
                        )),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        UpgradeText,
                    ));
                });
        });
}

fn show_ui(mut ui_query: Query<&mut Visibility, With<PrimaryGameUi>>) {
    for mut visibility in &mut ui_query {
        *visibility = Visibility::Visible;
    }
}

fn hide_ui(mut ui_query: Query<&mut Visibility, With<PrimaryGameUi>>) {
    for mut visibility in &mut ui_query {
        *visibility = Visibility::Hidden;
    }
}

fn update_ui(
    game_data: Res<GameData>,
    mut texts: ParamSet<(
        Single<&mut Text, With<CurrencyText>>,
        Single<&mut Text, With<UpgradeText>>,
    )>,
) {
    texts.p0().0 = format!("Currency: {:.1}", game_data.currency);

    texts.p1().0 = format!(
        "Buy AutoClicker ({:.1})\nOwned: {}",
        game_data.auto_clicker_cost, game_data.auto_clicker_count
    );
}

fn handle_click_button(
    mut game_data: ResMut<GameData>,
    interaction_query: Single<&Interaction, (Changed<Interaction>, With<ClickButton>)>,
) {
    match *interaction_query {
        Interaction::Pressed => {
            game_data.currency += game_data.click_power;
        }
        _ => {}
    }
}

fn handle_upgrade_button(
    mut commands: Commands,
    interaction_query: Single<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
) {
    match *interaction_query {
        Interaction::Pressed => {
            commands.trigger(UpgradeEvent::AutoClicker);
        }
        _ => {}
    }
}
