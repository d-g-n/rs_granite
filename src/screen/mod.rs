use std::collections::HashMap;

use crate::{
    screen,
    sprites::{SpriteAssets, SpriteSizes},
    GameState,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use self::structs::ScreenContext;

pub mod context;
mod render_screen;
pub mod structs;

pub(crate) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenContext::new(
            80,
            50,
            SpriteSizes {
                map_sprite_width: 16.,
                map_sprite_height: 16.,
                text_sprite_width: 8.,
                text_sprite_height: 16.,
            },
        ))
        .add_enter_system(GameState::Loading, render_screen::init_screen)
        .add_system(
            render_screen::render_screen
                .run_if(
                    move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                        GameState::InMenu { .. } => true,
                        GameState::InGame { .. } => true,
                        _ => false,
                    },
                )
                .label("render_screen"),
        );
    }
}
