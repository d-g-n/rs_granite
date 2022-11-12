use bevy::prelude::*;
use iyes_loopless::state::NextState;

use crate::{
    game_logic::{
        components::{Blocker, Monster, Position},
        map::{game_map::GameMap, pathfinding::astar_next_step},
        resources::PlayerResource,
    },
    GameState, InGameState,
};

pub fn handle_blocking_update(
    mut commands: Commands,
    player_res: Res<PlayerResource>,
    mut map: ResMut<GameMap>,
    blocker_position_query: Query<&Position, With<Blocker>>,
) {
    let mut new_blockers = vec![false; map.width * map.height];

    for blocker_pos in blocker_position_query.iter() {
        new_blockers[map.xy_idx_pos(blocker_pos)] = true;
    }

    map.blocking_tiles = new_blockers;
}
