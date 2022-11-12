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

pub fn handle_monster_movement(
    mut commands: Commands,
    player_res: Res<PlayerResource>,
    map: Res<GameMap>,

    mut mon_pos_query: Query<(Entity, &mut Position), With<Monster>>,
) {
    println!("handle mosnter");

    for (entity, mut mon_pos) in mon_pos_query.iter_mut() {
        let next_step_opt = astar_next_step(&map, mon_pos.clone(), player_res.cur_pos.clone());

        if let Some((mut next_step_vec, _)) = next_step_opt {
            
            if next_step_vec.len() >= 2 {
                // discard the head, it is itself
                next_step_vec.remove(0);

                let next_pos = next_step_vec[0].clone();

                println!("cur pos {} {}", mon_pos.x, mon_pos.y);
                println!("next {} {}", next_pos.x, next_pos.y);

                if !map.is_blocker(next_pos.x, next_pos.y) {
                    *mon_pos = next_step_vec[0].clone();
                }
            }
        }
    }

    commands.insert_resource(NextState(GameState::InGame {
        game_state: InGameState::AwaitingInput,
    }));
}
