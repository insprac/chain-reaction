use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{AppState, GameState};

const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::srgb(0.0, 0.63, 1.0);
const PRESSED_BUTTON: Color = Color::srgb(0.11, 0.3, 0.41);

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_menu)
            .add_systems(OnExit(AppState::InGame), unpause_game)
            .add_systems(
                Update,
                (
                    pause_game
                        .run_if(in_state(GameState::Running))
                        .run_if(input_just_pressed(KeyCode::Escape)),
                    unpause_game
                        .run_if(in_state(GameState::Paused))
                        .run_if(input_just_pressed(KeyCode::Escape)),
                    button_interaction.run_if(in_state(GameState::Paused)),
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
#[require(Button)]
pub enum PauseMenuButton {
    Resume,
    Menu,
    Exit,
}

fn pause_game(mut commands: Commands) {
    commands.set_state(GameState::Paused);
}

fn unpause_game(mut commands: Commands) {
    commands.set_state(GameState::Running);
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        PauseMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ZIndex(1000),
        children![
            create_button(PauseMenuButton::Resume, "Resume"),
            create_button(PauseMenuButton::Menu, "Menu"),
            create_button(PauseMenuButton::Exit, "Exit"),
        ],
    ));
}

fn cleanup_menu(mut commands: Commands, q_menu: Query<Entity, With<PauseMenu>>) -> Result {
    let entity = q_menu.single()?;
    commands.entity(entity).try_despawn();
    Ok(())
}

fn button_interaction(
    mut commands: Commands,
    mut q_interaction: Query<
        (&Interaction, &PauseMenuButton, &mut Button, &Children),
        Changed<Interaction>,
    >,
    mut q_text_color: Query<&mut TextColor>,
    mut evw_app_exit: EventWriter<AppExit>,
) -> Result {
    for (interaction, menu_button, mut button, children) in q_interaction.iter_mut() {
        let mut text_color = q_text_color.get_mut(children[0])?;

        match *interaction {
            Interaction::Pressed => {
                *text_color = TextColor(PRESSED_BUTTON);
                button.set_changed();

                match *menu_button {
                    PauseMenuButton::Resume => {
                        commands.set_state(GameState::Running);
                    }
                    PauseMenuButton::Exit => {
                        evw_app_exit.write(AppExit::Success);
                    }
                    PauseMenuButton::Menu => {
                        commands.set_state(AppState::Menu);
                    }
                }
            }
            Interaction::Hovered => {
                *text_color = TextColor(HOVERED_BUTTON);
                button.set_changed();
            }
            Interaction::None => {
                *text_color = TextColor(NORMAL_BUTTON);
            }
        }
    }

    Ok(())
}

fn create_button(button: PauseMenuButton, text: &str) -> impl Bundle + use<> {
    (button, children![Text::new(text), TextColor(NORMAL_BUTTON)])
}
