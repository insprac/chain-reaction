use bevy::prelude::*;

use crate::{
    AppState, GameState, building::BuildingSettings, game_assets::GameAssets, tower::TowerKind,
    waves::WaveManager,
};

const GREEN: Color = Color::srgb(0.15, 0.62, 0.33);
const GREY: Color = Color::srgb(0.2, 0.2, 0.2);

pub struct RewardSelectPlugin;

impl Plugin for RewardSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RewardSelect), setup_reward_select)
            .add_systems(OnExit(GameState::RewardSelect), cleanup_reward_select)
            .add_systems(
                Update,
                reward_button_interactions
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::RewardSelect)),
            );
    }
}

#[derive(Component)]
struct RewardSelectUi;

#[derive(Component)]
#[require(Button)]
struct RewardButton {
    reward: TowerKind,
}

fn reward_button_interactions(
    mut q_interaction: Query<(&RewardButton, &Interaction, &mut BorderColor, &mut Button)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut building_settings: ResMut<BuildingSettings>,
) {
    for (reward_button, interaction, mut border_color, mut button) in q_interaction.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                building_settings.towers.push(reward_button.reward.clone());
                next_game_state.set(GameState::Running);
                button.set_changed();
            }
            Interaction::Hovered => {
                *border_color = BorderColor(GREEN);
                button.set_changed();
            }
            Interaction::None => {
                *border_color = BorderColor(GREY);
            }
        }
    }
}

fn setup_reward_select(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    wave_manager: Res<WaveManager>,
) {
    let rewards = wave_manager.wave_reward().get_random_options();

    commands
        .spawn((
            RewardSelectUi,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            ZIndex(1000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Wave Complete!"),
                TextFont {
                    font: game_assets.audiowide_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Choose your reward"),
                TextFont {
                    font: game_assets.audiowide_font.clone(),
                    font_size: 30.0,
                    ..default()
                },
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                })
                .with_children(|parent| {
                    for reward in rewards {
                        let image = game_assets.tower_icons.get(&reward);

                        parent.spawn((
                            RewardButton { reward: reward },
                            Node {
                                width: Val::Px(60.0),
                                height: Val::Px(60.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            BorderColor(GREY),
                            BorderRadius::all(Val::Px(6.0)),
                            children![ImageNode { image, ..default() }],
                        ));
                    }
                });
        });
}

fn cleanup_reward_select(mut commands: Commands, id: Single<Entity, With<RewardSelectUi>>) {
    commands.entity(*id).despawn();
}
