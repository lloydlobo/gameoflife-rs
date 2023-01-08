//! This module implements the UI for the game of life.
//!
//! [Reference](https://github.com/bevyengine/bevy/blob/latest/examples/ui/button.rs)
//!
//! NOTE: Inverted y axis. everything goes from bottom-up, not top-down.
// FIXME: See if the above claim is true. inverted y axis?

use bevy::prelude::*;

// https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs
// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    /// Build function similar to that in main setup.
    fn build(&self, app: &mut App) {
        app.add_event::<GameExitEvent>()
            .add_event::<SimulationStartEvent>()
            .add_event::<SimulationStopEvent>()
            .add_startup_system(startup_system_setup)
            .add_system(button_system);
    }
}

//-------------------------------------------------------------------------------------------------
const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

pub struct GameExitEvent;

pub struct SimulationStartEvent;

pub struct SimulationStopEvent;

#[derive(Component)]
struct ClassicButton(ButtonType);

#[derive(PartialEq, Copy, Clone)]
enum ButtonType {
    Start,
    Stop,
    Exit,
}

/// A UI node that is a button
fn build_classic_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150f32), Val::Px(50f32)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        // ? deprecated: color.
        background_color: NORMAL_BUTTON.into(),
        image: UiImage(asset_server.load("sprites/button.png")),
        ..default()
    }
}

fn build_classic_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        // ? deprecated: `Text::with_section`
        text: Text::from_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/Symtext.ttf"),
                font_size: 30f32,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ),
        ..default()
    }
}

//-------------------------------------------------------------------------------------------------

// pub struct Query<'world, 'state, Q: WorldQuery, F: ReadOnlyWorldQuery = ()>
#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ClassicButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_writer: EventWriter<SimulationStartEvent>,
    mut stop_writer: EventWriter<SimulationStopEvent>,
    mut exit_writer: EventWriter<GameExitEvent>,
) {
    for (interaction, mut color, classic_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match classic_button.0 {
                    ButtonType::Start => {
                        start_writer.send(SimulationStartEvent);
                    }
                    ButtonType::Stop => {
                        stop_writer.send(SimulationStopEvent);
                    }
                    ButtonType::Exit => {
                        exit_writer.send(GameExitEvent);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------

/// * AssetServer is the backend that hosts the events like Start, Stop..
/// * Node bundle for root, that spans the entire game screen.
/// * Add `with_children` a child that spans bottom 100px entire width.
/// * Add bg panel.
/// * Border
/// * Fill.
fn startup_system_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera.
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            // root.
            style: {
                Style {
                    size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                }
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle /* Bottom button BG border. */ {
                    style: Style {
                        size: Size::new(Val::Percent(100f32), Val::Px(100f32)),
                        border: UiRect::all(Val::Px(5f32)), // ? deprecated Rect for border - parent
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle /* Bottom button BG fill. */ {
                            style: Style {
                                size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent /* PLAY BUTTON. */
                                .spawn(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_classic_text("PLAY", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Start));

                            parent /* STOP BUTTON. */
                                .spawn(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_classic_text("STOP", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Stop));

                            parent /* QUIT BUTTON. */
                                .spawn(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_classic_text("QUIT", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Exit));
                        });
                });
        });
}
