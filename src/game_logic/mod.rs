use std::collections::HashMap;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{screen::ScreenContext, GameState};

use self::components::{Position, Renderable};

mod components;
mod player;

pub(crate) struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_system(handle_renderable.run_if(
                move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                    GameState::InMenu { .. } => true,
                    GameState::InGame { .. } => true,
                    _ => false,
                },
            ));
    }
}

pub fn handle_renderable(
    mut commands: Commands,
    mut ctx: ResMut<ScreenContext>,
    mut query: Query<(Entity, &Position, &Renderable)>,
) {
    let mut position_visibility_history: HashMap<Position, f32> = HashMap::new();

    ctx.clear();

    for (entity, position, mut renderable) in query.iter_mut() {
        let mut screen_tile = ctx.get_tile(position.x as usize, position.y as usize);

        if !position_visibility_history.contains_key(position) {
            position_visibility_history.insert(position.clone(), renderable.layer);
        }

        if let Some(last_layer) = position_visibility_history.get(position) {
            if renderable.layer < *last_layer {
                continue;
            } else {
                position_visibility_history.insert(position.clone(), renderable.layer);
            }
        }

        screen_tile.fg_color = renderable.fg;
        screen_tile.bg_color = renderable.bg;
        screen_tile.glyph = renderable.glyph;
        screen_tile.visible = true;
        screen_tile.layer = renderable.layer
    }
}
