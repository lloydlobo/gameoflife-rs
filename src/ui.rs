//! This module implements the UI for the game of life.
//!
//! [Reference](https://github.com/bevyengine/bevy/blob/latest/examples/ui/button.rs)
//!
//! NOTE: Inverted y axis. everything goes from bottom-up, not top-down.

use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

impl Plugin for MainMenuPlugin {
    /// Build function similar to that in main setup.
    fn build(&self, app: &mut App) {
        app.add_event::<GameExitEvent>()
            .add_event::<SimulationStartEvent>()
            .add_event::<SimulationStopEvent>()
            .add_startup_system(startup_system_setup)
            .add_system(button_system);
    }
}

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

pub struct MainMenuPlugin;

//-------------------------------------------------------------------------------------------------

/// A UI node that is a button
fn build_classic_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style { ..default() },
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
                font_size: 30.0,
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

/// AssetServer is the backend that hosts the events like Start, Stop..
fn startup_system_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera.
    commands.spawn(Camera2dBundle::default());
}
