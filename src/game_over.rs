use bevy::prelude::*;

use crate::{AppState, game_assets::GameAssets, score::PlayerScore};

const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::srgb(0.0, 0.63, 1.0);
const PRESSED_BUTTON: Color = Color::srgb(0.11, 0.3, 0.41);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_game_over)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over)
            .add_systems(
                Update,
                button_interaction.run_if(in_state(AppState::GameOver)),
            );
    }
}

#[derive(Component)]
pub struct GameOverUi;

#[derive(Component)]
#[require(Button)]
pub enum GameOverButton {
    Play,
    Menu,
}

#[derive(Component)]
#[require(Camera2d)]
pub struct GameOverCamera;

fn button_interaction(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut q_interaction: Query<
        (
            &Interaction,
            &GameOverButton,
            &mut Button,
            &mut BorderColor,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut q_text_color: Query<&mut TextColor>,
) -> Result {
    for (interaction, game_over_button, mut button, mut border_color, children) in
        q_interaction.iter_mut()
    {
        let mut text_color = q_text_color.get_mut(children[0])?;

        match *interaction {
            Interaction::Pressed => {
                *text_color = TextColor(PRESSED_BUTTON);
                *border_color = BorderColor(PRESSED_BUTTON);
                button.set_changed();

                match *game_over_button {
                    GameOverButton::Play => {
                        next_app_state.set(AppState::InGame);
                    }
                    GameOverButton::Menu => {
                        next_app_state.set(AppState::Menu);
                    }
                }
            }
            Interaction::Hovered => {
                *text_color = TextColor(HOVERED_BUTTON);
                *border_color = BorderColor(HOVERED_BUTTON);
                button.set_changed();
            }
            Interaction::None => {
                *text_color = TextColor(NORMAL_BUTTON);
                *border_color = BorderColor(NORMAL_BUTTON);
            }
        }
    }

    Ok(())
}

fn setup_game_over(mut commands: Commands, game_assets: Res<GameAssets>, score: Res<PlayerScore>) {
    commands.spawn(GameOverCamera);

    commands.spawn((
        GameOverUi,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        },
        TextColor(Color::WHITE),
        children![
            (
                Text::new("Game Over"),
                TextFont {
                    font: game_assets.audiowide_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
            ),
            (
                Node {
                    width: Val::Px(300.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                children![
                    build_stat(format!("Score: {}", score.score), &game_assets),
                    build_stat(
                        format!("Highest combo: {}", score.highest_combo),
                        &game_assets
                    ),
                    build_stat(
                        format!("Highest chain kill: {}", score.highest_chain),
                        &game_assets
                    ),
                ]
            ),
            create_button(GameOverButton::Play, "Play again", &game_assets),
            create_button(GameOverButton::Menu, "Main menu", &game_assets),
        ],
    ));
}

fn cleanup_game_over(
    mut commands: Commands,
    ui_id: Single<Entity, With<GameOverUi>>,
    camera_id: Single<Entity, With<GameOverCamera>>,
) {
    commands.entity(*ui_id).despawn();
    commands.entity(*camera_id).despawn();
}

fn build_stat(label: impl Into<String>, game_assets: &GameAssets) -> impl Bundle {
    (
        Text::new(label),
        TextFont {
            font: game_assets.audiowide_font.clone(),
            font_size: 20.0,
            ..default()
        },
    )
}

fn create_button(
    button: GameOverButton,
    text: &str,
    game_assets: &GameAssets,
) -> impl Bundle + use<> {
    (
        button,
        Node {
            width: Val::Px(300.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor(NORMAL_BUTTON),
        BorderRadius::all(Val::Px(6.0)),
        children![
            Text::new(text),
            TextColor(NORMAL_BUTTON),
            TextFont {
                font: game_assets.audiowide_font.clone(),
                font_size: 30.0,
                ..default()
            }
        ],
    )
}
