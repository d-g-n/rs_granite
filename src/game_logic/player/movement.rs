use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    camera::MousePositionOnScreen,
    game_logic::{
        components::{Blocker, Player, Position, Viewshed},
        map::{game_map::GameMap, pathfinding::astar_next_step},
        resources::PlayerResource,
    },
    screen::structs::{ScreenContext, ScreenTilePriority},
    GameState, InGameState,
};

#[derive(Default)]
pub struct HeldCounter {
    counter_ms: u128,
    passed_initial_threshold: bool,
}

#[derive(Default)]
pub struct WaypointCounter {
    counter_ms: u128,
}

pub fn handle_player_movement(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut ctx: ResMut<ScreenContext>,
    time: Res<Time>,
    map: Res<GameMap>,
    mut held_counter: Local<HeldCounter>,
    mut waypoint_counter: Local<WaypointCounter>,
    mut player_res: ResMut<PlayerResource>,
    mut player_position_query: Query<(Entity, &mut Position, &mut Viewshed), With<Player>>,
    blocker_position_query: Query<(Entity, &Position), (With<Blocker>, Without<Player>)>,
) {
    let (mut direction_x, mut direction_y) =
        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            (0, 1)
        } else if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            (-1, 0)
        } else if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            (0, -1)
        } else if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            (1, 0)
        } else {
            if held_counter.counter_ms != 0 {
                *held_counter = HeldCounter::default();
            }

            (0, 0)
        };

    if direction_x != 0 || direction_y != 0 && !player_res.move_waypoints.is_empty() {
        player_res.move_waypoints.clear();
    }

    if !player_res.move_waypoints.is_empty() {
        let next_pos = &player_res.move_waypoints[0];

        waypoint_counter.counter_ms += time.delta().as_millis();

        if waypoint_counter.counter_ms > 200 {
            direction_x = next_pos.x - player_res.cur_pos.x;
            direction_y = next_pos.y - player_res.cur_pos.y;
        }
    }

    let (_entity, mut player_pos, mut viewshed) = player_position_query.single_mut();

    let new_x = player_pos.x + direction_x;
    let new_y = player_pos.y + direction_y;

    for (_entity, position) in blocker_position_query.iter() {
        if position.x == new_x && position.y == new_y {
            return;
        }
    }

    if new_x >= 0
        && new_y >= 0
        && new_x < ctx.width as i32
        && new_y < ctx.height as i32
        && held_counter.counter_ms == 0
        && (direction_x != 0 || direction_y != 0)
    {
        player_pos.x = new_x;
        player_pos.y = new_y;

        player_res.cur_pos = Position { x: new_x, y: new_y };

        if !player_res.move_waypoints.is_empty() {
            if waypoint_counter.counter_ms > 200 {
                player_res.move_waypoints.remove(0);
                waypoint_counter.counter_ms = 0;
            }
        }

        commands.insert_resource(NextState(GameState::InGame {
            game_state: InGameState::PlayerTurn,
        }));

        viewshed.dirty = true;
    }

    if !(direction_x == 0 && direction_y == 0) {
        held_counter.counter_ms += time.delta().as_millis();
    }

    if held_counter.passed_initial_threshold {
        if held_counter.counter_ms >= 100 {
            held_counter.counter_ms = 0;
        }
    } else {
        if held_counter.counter_ms >= 300 {
            held_counter.passed_initial_threshold = true;
        }
    }
}

pub fn handle_mouse_movement(
    mut player_res: ResMut<PlayerResource>,
    map: Res<GameMap>,
    mut ctx: ResMut<ScreenContext>,
    mut mouse_res: ResMut<MousePositionOnScreen>,
    mut pathfinding_history: Local<Vec<Position>>,
    buttons: Res<Input<MouseButton>>,
) {
    // if the mouse res changed, calculate a route and store it

    if mouse_res.is_changed() || player_res.is_changed() {
        if let Some(mouse_pos_map) = &mouse_res.mouse_pos_map_opt {
            let res = astar_next_step(
                &map,
                player_res.cur_pos.clone(),
                mouse_pos_map.to_position(),
            );

            // pop the head if it's > 0, as it'll be the players pos

            if let Some((mut pos_vec, _)) = res {
                if !pos_vec.is_empty() {
                    pos_vec.remove(0);
                }

                *pathfinding_history = pos_vec;
            }
        }
    }

    for ele in (*pathfinding_history).iter() {
        ctx.draw_glyph(
            ele.x as usize,
            ele.y as usize,
            ScreenTilePriority::Tooltip,
            |screen_tile| screen_tile.glyph.bg_color = Color::RED,
        );
    }

    if buttons.just_pressed(MouseButton::Left) {
        player_res.move_waypoints = pathfinding_history.clone();
    }
}
