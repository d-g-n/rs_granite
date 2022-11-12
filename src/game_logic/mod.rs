use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, InGameState};

use self::{rendering::handle_renderable, viewshed::handle_viewshed_updating};

pub mod components;
mod map;
mod monster;
mod player;
mod rendering;
mod resources;
mod viewshed;

pub(crate) struct GameLogicPlugin;
//handle_viewshed_updating
impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(monster::MonsterPlugin)
            .add_plugin(map::MapPlugin) //
            .add_system(
                handle_renderable
                    .run_if(
                        move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                            GameState::InGame {
                                game_state: InGameState::LoadMap,
                            } => false,
                            GameState::InMenu { .. } => true,
                            GameState::InGame { .. } => true,
                            _ => false,
                        },
                    )
                    .label("renderable_system")
                    .before("render_screen"),
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
