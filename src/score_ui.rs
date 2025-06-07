use bevy::{prelude::*, text::FontWeight};

use crate::{
    AppState,
    score::{ComboResetEvent, IncreaseScoreEvent, PlayerScore, ScoreSet},
};

pub struct ScoreUiPlugin;

impl Plugin for ScoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_score_ui)
            .add_systems(OnExit(AppState::InGame), cleanup_score_ui)
            .add_systems(
                Update,
                update_score
                    .after(ScoreSet)
                    .run_if(on_event::<IncreaseScoreEvent>.or(on_event::<ComboResetEvent>)),
            );
    }
}

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ComboText;

fn setup_score_ui(mut commands: Commands) {
    commands.spawn((
        ScoreUi,
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                ScoreText,
                Text::new("0"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
            ),
            (
                ComboText,
                Text::new(""),
                TextFont {
                    font_size: 30.0,
                    ..default()
                }
            )
        ],
    ));
}

fn cleanup_score_ui(mut commands: Commands, score_ui_id: Single<Entity, With<ScoreUi>>) {
    commands.entity(score_ui_id.into_inner()).despawn();
}

fn update_score(
    player_score: Res<PlayerScore>,
    score_text: Single<Entity, With<ScoreText>>,
    combo_text: Single<Entity, With<ComboText>>,
    mut writer: TextUiWriter,
) {
    if let Some(mut text) = writer.get_text(*score_text, 0) {
        *text = format!("{}", player_score.score);
    }
    if let Some(mut text) = writer.get_text(*combo_text, 0) {
        if player_score.combo == 0 {
            *text = String::new();
        } else {
            *text = format!("{}x", player_score.combo);
        }
    }
}
