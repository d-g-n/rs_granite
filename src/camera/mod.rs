use bevy::prelude::*;

use crate::{screen::ScreenContext, sprites::SpriteSizes};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, sprite_sizes: Res<SpriteSizes>, ctx: Res<ScreenContext>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.x = (ctx.width as f32 / 2.) * sprite_sizes.map_sprite_width;
    camera_bundle.transform.translation.y =
        (ctx.height as f32 / 2.) * sprite_sizes.map_sprite_height;
    commands.spawn_bundle(camera_bundle);
}
