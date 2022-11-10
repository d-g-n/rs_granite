use bevy::{prelude::*, render::camera::RenderTarget};
use iyes_loopless::prelude::*;

use crate::{
    screen::structs::ScreenContext, sprites::SpriteSizes, utils::Point, GameState, InGameState,
};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePositionOnScreen>()
            .add_startup_system(setup)
            .add_system(process_cursor_movement.run_if(
                move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                    GameState::InMenu { .. } => true,
                    GameState::InGame { .. } => true,
                    _ => false,
                },
            ));
    }
}

fn setup(mut commands: Commands, sprite_sizes: Res<SpriteSizes>, ctx: Res<ScreenContext>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.x = (ctx.width as f32 / 2.) * sprite_sizes.map_sprite_width;
    camera_bundle.transform.translation.y =
        (ctx.height as f32 / 2.) * sprite_sizes.map_sprite_height;
    commands.spawn_bundle(camera_bundle);
}

#[derive(Default, Debug)]
pub struct MousePositionOnScreen {
    pub mouse_pos_map_opt: Option<Point>,
    pub mouse_pos_text_opt: Option<Point>,
}

pub fn process_cursor_movement(
    // need to get window dimensions
    wnds: Res<Windows>,
    mut mouse_res: ResMut<MousePositionOnScreen>,
    sprite_sizes: Res<SpriteSizes>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let _world_pos: Vec2 = world_pos.truncate();

        let new_mouse_map_x = (world_pos.x / (sprite_sizes.map_sprite_width as f32)).trunc() as i32;
        let new_mouse_map_y =
            (world_pos.y / (sprite_sizes.map_sprite_height as f32)).trunc() as i32;

        match &mouse_res.mouse_pos_map_opt {
            Some(old_mouse_pos) => {
                if old_mouse_pos.x != new_mouse_map_x || old_mouse_pos.y != new_mouse_map_y {
                    mouse_res.mouse_pos_map_opt = Some(Point::new(new_mouse_map_x, new_mouse_map_y))
                }
            }
            None => {
                mouse_res.mouse_pos_map_opt = Some(Point::new(new_mouse_map_x, new_mouse_map_y))
            }
        }

        let new_mouse_text_x =
            (world_pos.x / (sprite_sizes.text_sprite_width as f32)).trunc() as i32;
        let new_mouse_text_y =
            (world_pos.y / (sprite_sizes.text_sprite_height as f32)).trunc() as i32;

        match &mouse_res.mouse_pos_text_opt {
            Some(old_mouse_pos) => {
                if old_mouse_pos.x != new_mouse_text_x || old_mouse_pos.y != new_mouse_text_y {
                    mouse_res.mouse_pos_text_opt =
                        Some(Point::new(new_mouse_text_x, new_mouse_text_y))
                }
            }
            None => {
                mouse_res.mouse_pos_text_opt = Some(Point::new(new_mouse_text_x, new_mouse_text_y))
            }
        }
    } else {
        if let Some(_) = mouse_res.mouse_pos_map_opt {
            mouse_res.mouse_pos_map_opt = None;
        }

        if let Some(_) = mouse_res.mouse_pos_text_opt {
            mouse_res.mouse_pos_text_opt = None;
        }
    }
}
