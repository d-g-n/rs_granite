use std::collections::HashMap;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{screen::ScreenContext, GameState, InGameState};

use self::{
    components::{Player, Position, Renderable, Viewshed},
    map::game_map::{GameMap, GameTile},
    viewshed::handle_viewshed_updating, rendering::handle_renderable,
};

mod components;
mod map;
mod player;
mod resources;
mod viewshed;
mod rendering;

pub(crate) struct GameLogicPlugin;
//handle_viewshed_updating
impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(map::MapPlugin)
            .add_system(
                handle_renderable.run_if(move |cur_state: Res<CurrentState<GameState>>| {
                    match cur_state.0 {
                        GameState::InGame {
                            game_state: InGameState::LoadMap,
                        } => false,
                        GameState::InMenu { .. } => true,
                        GameState::InGame { .. } => true,
                        _ => false,
                    }
                }),
            )
            .add_system(handle_viewshed_updating.run_if(
                move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                    GameState::InGame {
                        game_state: InGameState::LoadMap,
                    } => false,
                    GameState::InGame { .. } => true,
                    _ => false,
                },
            ));
    }
}

