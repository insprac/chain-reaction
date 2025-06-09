use bevy::prelude::*;

use crate::{AppState, game_assets::GameAssets};

const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::srgb(0.0, 0.63, 1.0);
const PRESSED_BUTTON: Color = Color::srgb(0.11, 0.3, 0.41);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnExit(AppState::Menu), cleanup_menu)
            .add_systems(Update, button_interaction.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
#[require(Button)]
pub enum MenuButton {
    Play,
    Exit,
}

#[derive(Component)]
pub struct MenuCamera;

fn setup_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Menu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        },
        children![
            (
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                Text::new("Chain Reaction"),
                TextFont {
                    font: game_assets.audiowide_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
            ),
            create_button(MenuButton::Play, "Play Game", &game_assets),
            create_button(MenuButton::Exit, "Exit", &game_assets),
        ],
    ));

    commands.spawn((MenuCamera, Camera2d));
}

fn cleanup_menu(
    mut commands: Commands,
    q_menu: Query<Entity, With<Menu>>,
    q_menu_camera: Query<Entity, With<MenuCamera>>,
) -> Result {
    let menu_entity = q_menu.single()?;
    commands.entity(menu_entity).try_despawn();

    let camera_entity = q_menu_camera.single()?;
    commands.entity(camera_entity).try_despawn();

    Ok(())
}

fn button_interaction(
    mut commands: Commands,
    mut q_interaction: Query<
        (
            &Interaction,
            &MenuButton,
            &mut Button,
            &mut BorderColor,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut q_text_color: Query<&mut TextColor>,
    mut evw_app_exit: EventWriter<AppExit>,
) -> Result {
    for (interaction, menu_button, mut button, mut border_color, children) in
        q_interaction.iter_mut()
    {
        let mut text_color = q_text_color.get_mut(children[0])?;

        match *interaction {
            Interaction::Pressed => {
                *text_color = TextColor(PRESSED_BUTTON);
                *border_color = BorderColor(PRESSED_BUTTON);
                button.set_changed();

                match *menu_button {
                    MenuButton::Play => {
                        commands.set_state(AppState::InGame);
                    }
                    MenuButton::Exit => {
                        evw_app_exit.write(AppExit::Success);
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

fn create_button(button: MenuButton, text: &str, game_assets: &GameAssets) -> impl Bundle {
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
