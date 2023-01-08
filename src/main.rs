//! This module implements Conway's game of life in `Bevy` game engine.
//!
//! Copied (with little modification) from [jonchisko/GameOfLife](https://github.com/jonchisko/GameOfLife)

mod input;
mod simulation;
mod ui;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use input::*;
use simulation::*;
use ui::*;

const GRID_SIZE: i32 = 100;

/// NOTE: Since bevy 0.9 - The `WindowDescriptor` settings have been moved
/// from a resource to `WindowPlugin::window`.
// [Setup](https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs)
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Game of Life".to_string(),
                width: 1024f32,
                height: 720f32,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // .add_plugins(SimulationPlugin)
        // .add_plugins(InputPlugin)
        // .add_plugins(MainMenuPlugin)
        .add_system(toggle_vsync)
        .run();
}

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
/// [Setup](https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs)
fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.just_pressed(KeyCode::V) {
        let window = windows.primary_mut();

        window.set_present_mode(if matches!(window.present_mode(), PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        });
        info!("PRESENT_MODE: {:?}", window.present_mode());
    }
}
