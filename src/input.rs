//! This module implements input for the game.
//!
//! `'w', 'a', 's', 'd'` - movement keys.
//! `'q', 'e'` to zoom in/out.
//!
//! [Reference](https://github.com/jonchisko/GameOfLife/blob/master/src/input.rs)

use std::ops::Neg;

use bevy::{
    prelude::*,
    time::FixedTimestep,
};

/// Max value for clamping camera move speed.
const CAMERA_MOVE_SPEED: f32 = 15f32;

/// Max value for clamping camera zoom speed.
const CAMERA_ZOOM_SPEED: f32 = 1f32;

/// Main camera of the game.
#[derive(Component)]
pub struct MainCamera;

/// Hold x, y direction plane camera move & zoom movement.
/// This is only on the `MainCamera`.
#[derive(Component)]
struct Movement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

type CameraMove<'a> = (Mut<'a, Transform>, Mut<'a, Movement>);
type CameraZoom<'a> = (Mut<'a, Movement>, Mut<'a, OrthographicProjection>);

/// The main input plugin handles all inputs.
pub struct InputPlugin;

/// Implements `Plugin` for getting a builder function for custom `InputPlugin`.
///
/// * `SystemSet` - run camera movement and zoom system at desired time interval with run criteria
///   functionality thorough a `FixedTimestep`.
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.033_f64))
                .with_system(camera_move)
                .with_system(camera_zoom),
        );
    }
}

//-------------------------------------------------------------------------------------------------

/// `camera_move`.
///
/// * `Query` - without width is enough with Transform since Only main camera has the movement
///   component
/// * `Movement` - tells which keys are pressed.
fn camera_move(
    mut camera: Query<(&mut Transform, &mut Movement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut move_direction = Vec3::new(0f32, 0f32, 0f32);
    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y += 1f32;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x += 1f32;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y -= 1f32;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x -= 1f32;
    }

    // Normalize the direction vector over zero, incase there is an attempt to divide by zero.
    let move_direction: Vec3 = move_direction.normalize_or_zero();

    // expect("Should transform on main camera");
    let (mut transform, mut movement): CameraMove = camera.iter_mut().next().unwrap();

    // Single object access as there is only one camera.
    movement.plane_speed = (movement.plane_speed + move_direction).clamp(
        // min.
        Vec3::new(CAMERA_MOVE_SPEED.neg(), -CAMERA_MOVE_SPEED.neg(), -CAMERA_MOVE_SPEED.neg()),
        // max.
        Vec3::new(CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED),
    );

    // Reset `plane_speed`.
    if keyboard_input.pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::new(0f32, 0f32, 0f32);
    }

    // Update translation. Velocity based translation.
    transform.translation += movement.plane_speed;
}

///
fn camera_zoom(
    mut camera: Query<(&mut Movement, &mut OrthographicProjection), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut zoom_direction = 0f32;
    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = 0.01f32;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = -0.01f32;
    }

    // camera.single_mut()
    let (mut movement, mut ortho_proj): CameraZoom = camera.iter_mut().next().unwrap();

    // Reset `zoom_speed`.
    if keyboard_input.pressed(KeyCode::Space) {
        movement.zoom_speed = 0.0f32;
    }

    movement.zoom_speed = (movement.zoom_speed + zoom_direction)
        .clamp(CAMERA_ZOOM_SPEED.neg(), CAMERA_ZOOM_SPEED.neg());
    ortho_proj.scale = (ortho_proj.scale + movement.zoom_speed).clamp(1f32, 6f32);

    // If zoom is at limit reset the zoom. so user doesn't have to traverse first towards zero and
    // to other direction. Don't need to flip polarity twice.
    let zoom_limit = 0.000_01_f32;
    let at_zoom_limit: bool = (ortho_proj.scale - 1f32).abs() < zoom_limit
        || (ortho_proj.scale - 6f32).abs() < zoom_limit;

    if at_zoom_limit {
        movement.zoom_speed = 0f32;
    }
}

//-------------------------------------------------------------------------------------------------

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera)
        .insert(Movement { plane_speed: Vec3::new(0f32, 0f32, 0f32), zoom_speed: 0f32 });
}
