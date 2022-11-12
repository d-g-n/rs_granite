use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, InGameState};

use self::update_blocking::handle_blocking_update;

pub mod builder;
pub mod builders;
pub mod game_map;
mod map_creation;
pub mod pathfinding;
mod update_blocking;

pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(
            GameState::InGame {
                game_state: InGameState::LoadMap,
            },
            map_creation::create_or_load_map,
        )
        .add_system(
            map_creation::visualise_map
                .run_in_state(GameState::InGame {
                    game_state: InGameState::LoadMap,
                })
                .before("render_screen"),
        )
        .add_exit_system(
            GameState::InGame {
                game_state: InGameState::LoadMap,
            },
            map_creation::finalise_map_creation,
        )
        .add_enter_system(
            GameState::InGame {
                game_state: InGameState::AwaitingInput,
            },
            handle_blocking_update,
        )
        .add_enter_system(
            GameState::InGame {
                game_state: InGameState::PlayerTurn,
            },
            handle_blocking_update,
        )
        .add_enter_system(
            GameState::InGame {
                game_state: InGameState::EnemyTurn,
            },
            handle_blocking_update,
        );
    }
}
