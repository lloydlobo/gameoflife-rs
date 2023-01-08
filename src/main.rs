//! This module implements Conway's game of life in `Bevy` game engine.
//!
//! Copied (with little modification) from [jonchisko/GameOfLife](https://github.com/jonchisko/GameOfLife)

mod input;
mod simulation;
mod ui;

use bevy::{
    diagnostic::{
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
    prelude::*,
    window::{
        CursorGrabMode,
        PresentMode,
    },
};
use input::*;
use simulation::*;
use ui::*;

const GRID_SIZE: i32 = 100;

/// One of the two settings that can be set through the menu. It will be a resource in the app
/// https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

/// One of the two settings that can be set through the menu. It will be a resource in the app
/// https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

//-------------------------------------------------------------------------------------------------

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
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // .add_plugin(SimulationPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(MenuPlugin)
        //bevy window stock systems.
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(change_title)
        .add_system(toggle_cursor)
        .add_system(toggle_vsync)
        // .add_system(cycle_cursor_icon)
        .run();
}

//-------------------------------------------------------------------------------------------------

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

/// This system will then change the title during execution
fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(format!(
        "Game of Life: Seconds since startup: {}",
        time.elapsed_seconds().round()
    ));
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_grab_mode(match window.cursor_grab_mode() {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        });
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

/// This system cycles the cursor's icon through a small set of icons when clicking
fn cycle_cursor_icon(
    input: Res<Input<MouseButton>>, mut windows: ResMut<Windows>, mut index: Local<usize>,
) {
    const ICONS: &[CursorIcon] = &[
        CursorIcon::Default,
        CursorIcon::Hand,
        CursorIcon::Wait,
        CursorIcon::Text,
        CursorIcon::Copy,
    ];
    let window = windows.primary_mut();
    if input.just_pressed(MouseButton::Left) {
        *index = (*index + 1) % ICONS.len();
        window.set_cursor_icon(ICONS[*index]);
    } else if input.just_pressed(MouseButton::Right) {
        *index = if *index == 0 { ICONS.len() - 1 } else { *index - 1 };
        window.set_cursor_icon(ICONS[*index]);
    }
}

//-------------------------------------------------------------------------------------------------
