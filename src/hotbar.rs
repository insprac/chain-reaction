use bevy::prelude::*;

use crate::{
    AppState, GameState,
    building::{BuildingSettings, BuildingsUpdatedEvent},
    game_assets::GameAssets,
    tower::TowerKind,
};

const GREEN: Color = Color::srgb(0.15, 0.62, 0.33);
const GREEN_HIGHLIGHT: Color = Color::srgb(0.09, 0.73, 0.34);
const GREY: Color = Color::srgb(0.2, 0.2, 0.2);

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_hotbar)
            .add_systems(OnExit(AppState::InGame), cleanup_hotbar)
            .add_systems(OnExit(GameState::RewardSelect), redraw_hotbar_images)
            .add_systems(
                Update,
                (
                    hotbar_interactions,
                    redraw_hotbar_images.run_if(on_event::<BuildingsUpdatedEvent>),
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running).or(in_state(GameState::Building))),
            );
    }
}

#[derive(Component)]
struct HotbarUi;

#[derive(Component)]
#[require(Button)]
struct HotbarButton {
    /// The index referring to `towers` index in `BuildingSettings`
    index: usize,
}

fn hotbar_interactions(
    mut q_interaction: Query<
        (&HotbarButton, &Interaction, &mut BorderColor, &mut Button),
        Changed<Interaction>,
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut building_settings: ResMut<BuildingSettings>,
) {
    for (hotbar_button, interaction, mut border_color, mut button) in q_interaction.iter_mut() {
        let has_tower = building_settings.towers.get(hotbar_button.index).is_some();
        match *interaction {
            Interaction::Pressed => {
                if has_tower {
                    building_settings.selected_tower = Some(hotbar_button.index);
                    next_game_state.set(GameState::Building);
                    *border_color = BorderColor(GREEN_HIGHLIGHT);
                    button.set_changed();
                }
            }
            Interaction::Hovered => {
                if has_tower {
                    *border_color = BorderColor(GREEN);
                    button.set_changed();
                }
            }
            Interaction::None => {
                *border_color = BorderColor(GREY);
            }
        }
    }
}

fn setup_hotbar(
    mut commands: Commands,
    building_settings: ResMut<BuildingSettings>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        HotbarUi,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(20.0),
            ..default()
        },
        ZIndex(500),
        children![
            hotbar_button(0, &game_assets, &building_settings),
            hotbar_button(1, &game_assets, &building_settings),
            hotbar_button(2, &game_assets, &building_settings),
            hotbar_button(3, &game_assets, &building_settings),
            hotbar_button(4, &game_assets, &building_settings),
        ],
    ));
}

fn cleanup_hotbar(mut commands: Commands, id: Single<Entity, With<HotbarUi>>) {
    commands.entity(*id).despawn();
}

fn redraw_hotbar_images(
    q_hotbar_button: Query<(&HotbarButton, &Children)>,
    mut q_image_node: Query<&mut ImageNode>,
    game_assets: Res<GameAssets>,
    building_settings: Res<BuildingSettings>,
) -> Result {
    for (hotbar_button, children) in q_hotbar_button {
        let mut image_node = q_image_node.get_mut(*children.first().unwrap())?;
        image_node.image = building_settings
            .towers
            .get(hotbar_button.index)
            .map(|t| game_assets.tower_icons.get(t))
            .unwrap_or_else(|| game_assets.tower_empty_icon.clone());
    }

    Ok(())
}

fn hotbar_button(
    index: usize,
    game_assets: &GameAssets,
    building_settings: &BuildingSettings,
) -> impl Bundle {
    let image = building_settings
        .towers
        .get(index)
        .map(|t| game_assets.tower_icons.get(t))
        .unwrap_or_else(|| game_assets.tower_empty_icon.clone());

    return (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                HotbarButton { index },
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
                children![ImageNode { image, ..default() }]
            ),
            (
                Text::new((index + 1).to_string()),
                TextFont {
                    font: game_assets.audiowide_font.clone(),
                    font_size: 20.0,
                    ..default()
                },
            )
        ],
    );
}
