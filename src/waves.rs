use bevy::prelude::*;
use hexx::Hex;
use rand::seq::IndexedRandom;

use crate::{
    AppState, GameState,
    arena::Arena,
    enemy::{Enemy, EnemySet, SpawnEnemyCommand},
    tower::TowerKind,
};

pub const WAVES: &[Wave] = &[
    // Wave 1
    Wave {
        stages: &[
            WaveStage {
                enemies: 2,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 3,
                remaining_threshold: 1,
            },
            WaveStage {
                enemies: 4,
                remaining_threshold: 2,
            },
        ],
        reward: WaveReward {
            options: 1,
            pool: &[
                TowerKind::Bullet2,
                TowerKind::Bullet3,
                TowerKind::Explosion1,
            ],
        },
    },
    // Wave 2
    Wave {
        stages: &[
            WaveStage {
                enemies: 4,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 6,
                remaining_threshold: 2,
            },
            WaveStage {
                enemies: 6,
                remaining_threshold: 4,
            },
            WaveStage {
                enemies: 6,
                remaining_threshold: 4,
            },
        ],
        reward: WaveReward {
            options: 2,
            pool: &[
                TowerKind::Bullet2,
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Explosion1,
            ],
        },
    },
    // Wave 3
    Wave {
        stages: &[
            WaveStage {
                enemies: 6,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 8,
                remaining_threshold: 4,
            },
            WaveStage {
                enemies: 8,
                remaining_threshold: 6,
            },
            WaveStage {
                enemies: 10,
                remaining_threshold: 8,
            },
        ],
        reward: WaveReward {
            options: 2,
            pool: &[
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Explosion1,
            ],
        },
    },
    // Wave 4
    Wave {
        stages: &[
            WaveStage {
                enemies: 8,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 10,
                remaining_threshold: 6,
            },
            WaveStage {
                enemies: 12,
                remaining_threshold: 8,
            },
            WaveStage {
                enemies: 12,
                remaining_threshold: 10,
            },
            WaveStage {
                enemies: 14,
                remaining_threshold: 12,
            },
            WaveStage {
                enemies: 16,
                remaining_threshold: 12,
            },
        ],
        reward: WaveReward {
            options: 3,
            pool: &[
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Bullet6,
                TowerKind::Explosion1,
                TowerKind::Explosion2,
            ],
        },
    },
    // Wave 5
    Wave {
        stages: &[
            WaveStage {
                enemies: 10,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 12,
                remaining_threshold: 8,
            },
            WaveStage {
                enemies: 14,
                remaining_threshold: 10,
            },
            WaveStage {
                enemies: 16,
                remaining_threshold: 12,
            },
            WaveStage {
                enemies: 18,
                remaining_threshold: 14,
            },
            WaveStage {
                enemies: 20,
                remaining_threshold: 16,
            },
        ],
        reward: WaveReward {
            options: 3,
            pool: &[
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Bullet6,
                TowerKind::Explosion2,
                TowerKind::Explosion3,
            ],
        },
    },
    // Wave 6
    Wave {
        stages: &[
            WaveStage {
                enemies: 14,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 16,
                remaining_threshold: 10,
            },
            WaveStage {
                enemies: 18,
                remaining_threshold: 12,
            },
            WaveStage {
                enemies: 20,
                remaining_threshold: 14,
            },
            WaveStage {
                enemies: 22,
                remaining_threshold: 16,
            },
            WaveStage {
                enemies: 24,
                remaining_threshold: 18,
            },
            WaveStage {
                enemies: 26,
                remaining_threshold: 18,
            },
        ],
        reward: WaveReward {
            options: 3,
            pool: &[
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Bullet6,
                TowerKind::Explosion2,
                TowerKind::Explosion3,
            ],
        },
    },
    // Wave 7
    Wave {
        stages: &[
            WaveStage {
                enemies: 18,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 20,
                remaining_threshold: 14,
            },
            WaveStage {
                enemies: 22,
                remaining_threshold: 16,
            },
            WaveStage {
                enemies: 24,
                remaining_threshold: 18,
            },
            WaveStage {
                enemies: 26,
                remaining_threshold: 20,
            },
            WaveStage {
                enemies: 32,
                remaining_threshold: 22,
            },
            WaveStage {
                enemies: 34,
                remaining_threshold: 22,
            },
            WaveStage {
                enemies: 38,
                remaining_threshold: 26,
            },
        ],
        reward: WaveReward {
            options: 3,
            pool: &[
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Bullet6,
                TowerKind::Explosion2,
                TowerKind::Explosion3,
            ],
        },
    },
    // Wave 8
    Wave {
        stages: &[
            WaveStage {
                enemies: 22,
                remaining_threshold: 0,
            },
            WaveStage {
                enemies: 24,
                remaining_threshold: 18,
            },
            WaveStage {
                enemies: 26,
                remaining_threshold: 20,
            },
            WaveStage {
                enemies: 28,
                remaining_threshold: 22,
            },
            WaveStage {
                enemies: 34,
                remaining_threshold: 24,
            },
            WaveStage {
                enemies: 36,
                remaining_threshold: 26,
            },
            WaveStage {
                enemies: 38,
                remaining_threshold: 26,
            },
            WaveStage {
                enemies: 42,
                remaining_threshold: 30,
            },
            WaveStage {
                enemies: 50,
                remaining_threshold: 40,
            },
        ],
        reward: WaveReward {
            options: 3,
            pool: &[
                TowerKind::Bullet3,
                TowerKind::Bullet4,
                TowerKind::Bullet6,
                TowerKind::Explosion2,
                TowerKind::Explosion3,
            ],
        },
    },
];

pub struct Wave {
    pub stages: &'static [WaveStage],
    pub reward: WaveReward,
}

pub struct WaveStage {
    /// How many enemies to spawn, they all spawn at once around the edge of the arena.
    pub enemies: usize,
    /// Enemies wont spawn until there is at most this many enemies remaining.
    pub remaining_threshold: usize,
}

pub struct WaveReward {
    /// The number of options that will appear for the player to select from.
    pub options: usize,
    /// Rewards will be randomly selected from this pool.
    pub pool: &'static [TowerKind],
}

impl WaveReward {
    pub fn get_random_options(&self) -> Vec<TowerKind> {
        let mut rng = rand::rng();
        self.pool
            .choose_multiple(&mut rng, self.options)
            .map(Clone::clone)
            .collect()
    }
}

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveManager>()
            .add_event::<WaveStartedEvent>()
            .add_event::<WaveStageStartedEvent>()
            .add_systems(OnEnter(AppState::InGame), setup_waves.in_set(EnemySet))
            .add_systems(
                OnTransition {
                    exited: GameState::RewardSelect,
                    entered: GameState::Running,
                },
                next_wave,
            )
            .add_systems(
                Update,
                (
                    update_wave_progress,
                    spawn_stage.run_if(on_event::<WaveStageStartedEvent>),
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Resource)]
pub struct WaveManager {
    wave: usize,
    stage: usize,
    update_timer: Timer,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            wave: 0,
            stage: 0,
            update_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl WaveManager {
    /// The display value of the wave starting from 1.
    pub fn wave_display(&self) -> usize {
        self.wave + 1
    }

    /// The display value of the wave stage starting from 1.
    pub fn stage_display(&self) -> usize {
        self.stage + 1
    }

    pub fn wave_reward(&self) -> &'static WaveReward {
        &WAVES[self.wave].reward
    }
}

#[derive(Event)]
pub struct WaveStartedEvent {
    /// The wave that just started (1, 2, 3, ...).
    pub wave: usize,
}

#[derive(Event)]
pub struct WaveStageStartedEvent {
    /// The current wave.
    pub wave: usize,
    /// The stage that just started.
    pub stage: usize,
}

fn setup_waves(
    mut wave_manager: ResMut<WaveManager>,
    mut evw_wave_started: EventWriter<WaveStartedEvent>,
    mut evw_wave_stage_started: EventWriter<WaveStageStartedEvent>,
) {
    wave_manager.wave = 0;
    wave_manager.stage = 0;
    evw_wave_started.write(WaveStartedEvent {
        wave: wave_manager.wave_display(),
    });
    evw_wave_stage_started.write(WaveStageStartedEvent {
        wave: wave_manager.wave_display(),
        stage: wave_manager.stage_display(),
    });
}

fn spawn_stage(mut commands: Commands, arena: Res<Arena>, wave_manager: Res<WaveManager>) {
    let wave_stage = &WAVES[wave_manager.wave].stages[wave_manager.stage];

    // Get the hexes around the edge of the arena to distribute randomly among them
    let mut hexes: Vec<Hex> = Hex::ZERO.ring(Arena::RADIUS).collect();

    for _ in 0..wave_stage.enemies {
        if hexes.len() == 0 {
            // Refill hexes and continue
            hexes = Hex::ZERO.ring(Arena::RADIUS).collect();
        }
        let index = rand::random_range(0..hexes.len() - 1);
        let hex = hexes.swap_remove(index);
        commands.queue(SpawnEnemyCommand::new(arena.layout.hex_to_world_pos(hex)));
    }
}

fn update_wave_progress(
    time: Res<Time>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut wave_manager: ResMut<WaveManager>,
    mut evw_wave_stage_started: EventWriter<WaveStageStartedEvent>,
    q_enemies: Query<Entity, With<Enemy>>,
) {
    // Only check for updates once per timer
    wave_manager.update_timer.tick(time.delta());
    if !wave_manager.update_timer.just_finished() {
        return;
    }

    // Get a total count of enemies
    let remaining_enemies = q_enemies.iter().len();

    let wave = &WAVES[wave_manager.wave];
    if wave.stages.len() <= wave_manager.stage + 1 {
        // This is the last stage
        if remaining_enemies > 0 {
            return;
        }

        if WAVES.len() <= wave_manager.wave + 1 {
            // There are no more waves, end the game
            next_app_state.set(AppState::GameOver);
            return;
        }

        next_game_state.set(GameState::RewardSelect);
    } else {
        // There are stages remaining
        let stage = &wave.stages[wave_manager.stage];
        if remaining_enemies > stage.remaining_threshold {
            return;
        }
        wave_manager.stage += 1;

        evw_wave_stage_started.write(WaveStageStartedEvent {
            wave: wave_manager.wave_display(),
            stage: wave_manager.stage_display(),
        });
    }
}

fn next_wave(
    mut wave_manager: ResMut<WaveManager>,
    mut evw_wave_started: EventWriter<WaveStartedEvent>,
    mut evw_wave_stage_started: EventWriter<WaveStageStartedEvent>,
) {
    wave_manager.wave += 1;
    wave_manager.stage = 0;

    evw_wave_started.write(WaveStartedEvent {
        wave: wave_manager.wave_display(),
    });
    evw_wave_stage_started.write(WaveStageStartedEvent {
        wave: wave_manager.wave_display(),
        stage: wave_manager.stage_display(),
    });
}
