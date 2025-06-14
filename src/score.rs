use bevy::prelude::*;

use crate::{AppState, GameState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IncreaseScoreEvent>()
            .add_event::<ComboResetEvent>()
            .init_resource::<PlayerScore>()
            .add_systems(OnEnter(AppState::InGame), reset_player_score)
            .add_systems(
                Update,
                (
                    tick_combo.run_if(not(on_event::<IncreaseScoreEvent>)),
                    increase_score.run_if(on_event::<IncreaseScoreEvent>),
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScoreSet;

/// Tracks the player's total score, combo, and other stats.
#[derive(Resource)]
pub struct PlayerScore {
    /// The player's total score.
    pub score: u128,
    /// The current combo, each hit is multiplied by it.
    pub combo: u128,
    /// Combo resets when this timer ends, timer resets each time the score increases.
    pub combo_timer: Timer,
    /// The highest combo the player has achieved.
    pub highest_combo: u128,
    /// The largest chain of tower triggers to be scored, for gameover stats.
    pub highest_chain: usize,
}

impl Default for PlayerScore {
    fn default() -> Self {
        Self {
            score: 0,
            combo: 0,
            combo_timer: Timer::from_seconds(1.0, TimerMode::Once),
            highest_combo: 0,
            highest_chain: 0,
        }
    }
}

/// Emit this to increase the player's score.
#[derive(Event)]
pub struct IncreaseScoreEvent {
    /// The amount to increase the score (pre-modifiers).
    pub score: u128,
    /// The amount of triggers chained before causing the score.
    pub chain_length: usize,
}

/// Emitted when the player's combo is reset.
#[derive(Event)]
pub struct ComboResetEvent;

fn reset_player_score(mut player_score: ResMut<PlayerScore>) {
    *player_score = PlayerScore::default();
}

fn tick_combo(
    time: Res<Time>,
    mut player_score: ResMut<PlayerScore>,
    mut evw_combo_reset: EventWriter<ComboResetEvent>,
) {
    player_score.combo_timer.tick(time.delta());
    if player_score.combo_timer.finished() {
        player_score.combo = 0;
        evw_combo_reset.write(ComboResetEvent);
    }
}

fn increase_score(
    mut player_score: ResMut<PlayerScore>,
    mut evr_increase_score: EventReader<IncreaseScoreEvent>,
) {
    player_score.combo_timer.reset();

    for event in evr_increase_score.read() {
        let chain_mult = 1 + event.chain_length as u128;
        let combo_mult = player_score.combo.max(1);
        player_score.score += event.score * chain_mult * combo_mult;
        player_score.combo += 1;

        if player_score.combo > player_score.highest_combo {
            player_score.highest_combo = player_score.combo;
        }

        if event.chain_length > player_score.highest_chain {
            player_score.highest_chain = event.chain_length;
        }
    }
}
